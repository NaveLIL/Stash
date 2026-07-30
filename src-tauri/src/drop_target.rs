#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
    Win32::System::Com::StructuredStorage::*,
    Win32::System::Ole::*,
    Win32::System::SystemServices::*,
    Win32::System::Memory::*,
    Win32::UI::Shell::*,
};
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "windows")]
use crate::DropPayload;

#[cfg(target_os = "windows")]
#[implement(IDropTarget)]
pub struct DropTarget {
    hwnd: HWND,
    app_handle: AppHandle,
}

#[cfg(target_os = "windows")]
impl DropTarget {
    pub fn new(hwnd: HWND, app_handle: AppHandle) -> Self {
        Self { hwnd, app_handle }
    }
    
    pub fn register(&self) -> Result<()> {
        unsafe {
            let _ = OleInitialize(None);
            let target: IDropTarget = self.into();
            RegisterDragDrop(self.hwnd, &target)?;
            Ok(())
        }
    }
}

#[cfg(target_os = "windows")]
impl IDropTarget_Impl for DropTarget {
    fn DragEnter(
        &self,
        _pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        _pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> Result<()> {
        unsafe { *pdweffect = DROPEFFECT_COPY; }
        Ok(())
    }

    fn DragOver(
        &self,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        _pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> Result<()> {
        unsafe { *pdweffect = DROPEFFECT_COPY; }
        Ok(())
    }

    fn DragLeave(&self) -> Result<()> {
        Ok(())
    }

    fn Drop(
        &self,
        pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        _pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> Result<()> {
        unsafe { *pdweffect = DROPEFFECT_COPY; }
        
        if let Some(data_obj) = pdataobj {
            let payloads = parse_data_object(data_obj);
            for payload in payloads {
                let _ = self.app_handle.emit("stash://item-dropped", payload);
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn parse_data_object(data_obj: &IDataObject) -> Vec<DropPayload> {
    let mut payloads = Vec::new();
    
    // Check for CF_HDROP (Files)
    let mut format_hdrop = FORMATETC {
        cfFormat: CF_HDROP.0 as u16,
        ptd: std::ptr::null_mut(),
        dwAspect: DVASPECT_CONTENT.0 as u32,
        lindex: -1,
        tymed: TYMED_HGLOBAL.0 as u32,
    };
    
    unsafe {
        if data_obj.QueryGetData(&format_hdrop).is_ok() {
            if let Ok(mut stg) = data_obj.GetData(&format_hdrop) {
                let hdrop = HDROP(stg.u.hGlobal.0 as _);
                let count = DragQueryFileW(hdrop, 0xFFFFFFFF, None);
                for i in 0..count {
                    let len = DragQueryFileW(hdrop, i, None) as usize + 1;
                    let mut buf = vec![0u16; len];
                    DragQueryFileW(hdrop, i, Some(&mut buf));
                    let path = String::from_utf16_lossy(&buf[..len - 1]);
                    
                    payloads.push(DropPayload {
                        id: uuid::Uuid::new_v4().to_string(),
                        item_type: "file".to_string(),
                        content: path.clone(),
                        preview_path: Some(path), // Basic preview via path
                    });
                }
                ReleaseStgMedium(&mut stg);
            }
        }
    }

    // Check for CF_UNICODETEXT
    let mut format_text = FORMATETC {
        cfFormat: CF_UNICODETEXT.0 as u16,
        ptd: std::ptr::null_mut(),
        dwAspect: DVASPECT_CONTENT.0 as u32,
        lindex: -1,
        tymed: TYMED_HGLOBAL.0 as u32,
    };

    unsafe {
        if data_obj.QueryGetData(&format_text).is_ok() {
            if let Ok(mut stg) = data_obj.GetData(&format_text) {
                let ptr = GlobalLock(stg.u.hGlobal);
                if !ptr.is_null() {
                    let size = GlobalSize(stg.u.hGlobal) / 2;
                    let slice = std::slice::from_raw_parts(ptr as *const u16, size);
                    let mut text = String::from_utf16_lossy(slice);
                    text = text.trim_matches(char::from(0)).to_string();
                    GlobalUnlock(stg.u.hGlobal);
                    
                    // Basic heuristic: check if URL
                    let item_type = if text.starts_with("http://") || text.starts_with("https://") { "url" } else { "text" };
                    
                    if !payloads.iter().any(|p| p.content == text) {
                        payloads.push(DropPayload {
                            id: uuid::Uuid::new_v4().to_string(),
                            item_type: item_type.to_string(),
                            content: text,
                            preview_path: None,
                        });
                    }
                }
                ReleaseStgMedium(&mut stg);
            }
        }
    }
    
    payloads
}
