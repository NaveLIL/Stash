use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[tauri::command]
pub async fn compress_image(path: String) -> Result<String, String> {
    let img = image::open(&path).map_err(|e| e.to_string())?;
    
    let temp_dir = std::env::temp_dir();
    let file_name = format!("{}_compressed.jpg", uuid::Uuid::new_v4());
    let out_path = temp_dir.join(file_name);
    
    // Save as JPEG with default compression
    img.save_with_format(&out_path, image::ImageFormat::Jpeg).map_err(|e| e.to_string())?;
    
    Ok(out_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn create_zip(paths: Vec<String>) -> Result<String, String> {
    let temp_dir = std::env::temp_dir();
    let file_name = format!("stash_{}.zip", uuid::Uuid::new_v4());
    let out_path = temp_dir.join(&file_name);
    
    let file = File::create(&out_path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    
    for path_str in paths {
        let path = Path::new(&path_str);
        if path.is_file() {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            let mut f = File::open(path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }
    
    zip.finish().map_err(|e| e.to_string())?;
    
    Ok(out_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn clean_url(url_str: String) -> String {
    if let Ok(mut url) = url::Url::parse(&url_str) {
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (k, v) in url.query_pairs() {
            if !k.starts_with("utm_") {
                pairs.push((k.into_owned(), v.into_owned()));
            }
        }
        url.query_pairs_mut().clear().extend_pairs(pairs);
        url.to_string()
    } else {
        url_str
    }
}

#[tauri::command]
pub async fn generate_qr(url_str: String) -> Result<String, String> {
    use qrcode::QrCode;
    use image::Luma;
    
    let code = QrCode::new(url_str.as_bytes()).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    
    let temp_dir = std::env::temp_dir();
    let file_name = format!("qr_{}.png", uuid::Uuid::new_v4());
    let out_path = temp_dir.join(file_name);
    
    image.save(&out_path).map_err(|e| e.to_string())?;
    
    Ok(out_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_url_utm_params() {
        let url = "https://example.com/product?utm_source=test&q=search&utm_medium=email".to_string();
        let expected = "https://example.com/product?q=search";
        assert_eq!(clean_url(url), expected);
    }

    #[test]
    fn test_clean_url_no_params() {
        let url = "https://example.com/".to_string();
        assert_eq!(clean_url(url.clone()), url);
    }

    #[test]
    fn test_clean_url_invalid() {
        let url = "not_a_valid_url".to_string();
        assert_eq!(clean_url(url.clone()), url);
    }

    #[test]
    fn test_generate_qr() {
        let result = generate_qr("https://github.com/NaveLIL".to_string());
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(std::path::Path::new(&path).exists());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_create_zip() {
        // Create dummy files
        let temp_dir = std::env::temp_dir();
        let file1 = temp_dir.join("stash_test_file1.txt");
        let file2 = temp_dir.join("stash_test_file2.txt");
        std::fs::write(&file1, b"Hello World 1").unwrap();
        std::fs::write(&file2, b"Hello World 2").unwrap();

        let paths = vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string()
        ];
        
        let result = create_zip(paths);
        assert!(result.is_ok());
        let zip_path = result.unwrap();
        assert!(std::path::Path::new(&zip_path).exists());
        assert!(std::fs::metadata(&zip_path).unwrap().len() > 0);

        std::fs::remove_file(file1).unwrap();
        std::fs::remove_file(file2).unwrap();
        std::fs::remove_file(zip_path).unwrap();
    }

    #[test]
    fn test_compress_image() {
        // Create a heavy dummy image (2000x2000)
        let mut img = image::ImageBuffer::new(2000, 2000);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("stash_test_heavy.png");
        img.save(&src_path).unwrap();

        let original_size = std::fs::metadata(&src_path).unwrap().len();

        let result = compress_image(src_path.to_string_lossy().to_string());
        assert!(result.is_ok());
        let compressed_path = result.unwrap();

        let compressed_size = std::fs::metadata(&compressed_path).unwrap().len();
        
        // Assert it actually compressed
        assert!(compressed_size < original_size);

        std::fs::remove_file(src_path).unwrap();
        std::fs::remove_file(compressed_path).unwrap();
    }
}
