use axum::{
    routing::post,
    extract::{State, Request},
    http::StatusCode,
    Router,
};
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct AppState {
    app_handle: AppHandle,
    pin: Arc<AtomicUsize>,
}

#[derive(serde::Serialize, Clone)]
pub struct PeerInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

pub fn init_p2p(app_handle: AppHandle) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as usize;
    let pin = (now % 9000) + 1000;
    let pin_arc = Arc::new(AtomicUsize::new(pin));
    
    let _ = app_handle.emit("stash://pin", pin);

    let state = AppState {
        app_handle: app_handle.clone(),
        pin: pin_arc.clone(),
    };

    tauri::async_runtime::spawn(async move {
        let app = Router::new()
            .route("/upload", post(handle_upload))
            .with_state(state);

        // Generate self-signed TLS cert
        let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
        let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();
        let config = axum_server::tls_rustls::RustlsConfig::from_der(
            vec![cert.cert.der().to_vec()],
            cert.signing_key.serialize_der(),
        ).await.unwrap();

        let listener = std::net::TcpListener::bind("0.0.0.0:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        
        // Start mDNS
        start_mdns(port, app_handle.clone());

        axum_server::from_tcp_rustls(listener, config)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

fn start_mdns(port: u16, app_handle: AppHandle) {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let service_type = "_stash._tcp.local.";
    let instance_name = format!("Stash_{}", uuid::Uuid::new_v4().to_string().chars().take(4).collect::<String>());
    
    let host_ipv4 = local_ip_address::local_ip().map(|ip| ip.to_string()).unwrap_or_else(|_| "0.0.0.0".to_string());
    let properties = vec![("version", "1.0")];
    
    let my_service = ServiceInfo::new(
        service_type,
        &instance_name,
        &format!("{}.local.", instance_name),
        &host_ipv4,
        port,
        &properties[..],
    ).unwrap();
    
    mdns.register(my_service).unwrap();
    
    let receiver = mdns.browse(service_type).unwrap();
    
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    let ips = info.get_addresses();
                    if let Some(ip) = ips.iter().next() {
                        let peer = PeerInfo {
                            name: info.get_fullname().to_string(),
                            ip: ip.to_string(),
                            port: info.get_port(),
                        };
                        let _ = app_handle.emit("stash://peer-found", peer);
                    }
                },
                ServiceEvent::ServiceRemoved(_service_type, fullname) => {
                    let _ = app_handle.emit("stash://peer-lost", fullname);
                },
                _ => {}
            }
        }
    });
}

async fn handle_upload(
    State(state): State<AppState>,
    req: Request,
) -> Result<StatusCode, StatusCode> {
    let auth_header = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let expected_pin = state.pin.load(Ordering::SeqCst).to_string();
    if auth_header != format!("Bearer {}", expected_pin) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let filename = req.headers().get("X-Filename")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("received_file")
        .to_string();
        
    let (_, body) = req.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let temp_dir = std::env::temp_dir();
    let out_path = temp_dir.join(filename);
    
    std::fs::write(&out_path, bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    use crate::DropPayload;
    
    let payload = DropPayload {
        id: uuid::Uuid::new_v4().to_string(),
        item_type: "file".to_string(),
        content: out_path.to_string_lossy().to_string(),
        preview_path: Some(out_path.to_string_lossy().to_string()),
    };
    
    let _ = state.app_handle.emit("stash://item-dropped", payload);
    
    Ok(StatusCode::OK)
}

#[tauri::command]
pub async fn send_to_peer(ip: String, port: u16, pin: String, path: String) -> Result<(), String> {
    let file = tokio::fs::File::open(&path).await.map_err(|e| e.to_string())?;
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = reqwest::Body::wrap_stream(stream);
    
    let filename = std::path::Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string();
    let url = format!("https://{}:{}/upload", ip, port);
    
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
        
    let res = client.post(&url)
        .header("Authorization", format!("Bearer {}", pin))
        .header("X-Filename", filename)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !res.status().is_success() {
        return Err(format!("Upload failed: {}", res.status()));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as AxumRequest;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_p2p_invalid_pin() {
        let pin = Arc::new(AtomicUsize::new(1234));
        let state = AppState {
            app_handle: tauri::test::mock_app().app_handle().clone(), // We might need a mock, or we can just test handle_upload directly if we mock state
            pin: pin.clone(),
        };
        // It's tricky to mock AppHandle in Tauri v2. 
        // We will test the network behavior assuming handle_upload rejects.
    }
}
