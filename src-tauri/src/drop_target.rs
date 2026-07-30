#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
    Win32::System::Ole::*,
};

#[cfg(target_os = "windows")]
#[implement(IDropTarget)]
pub struct DropTarget {
    hwnd: HWND,
}

#[cfg(target_os = "windows")]
impl DropTarget {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }
    
    pub fn register(&self) -> Result<()> {
        unsafe {
            // Need to initialize COM first
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
        unsafe {
            *pdweffect = DROPEFFECT_COPY;
        }
        Ok(())
    }

    fn DragOver(
        &self,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        _pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> Result<()> {
        unsafe {
            *pdweffect = DROPEFFECT_COPY;
        }
        Ok(())
    }

    fn DragLeave(&self) -> Result<()> {
        Ok(())
    }

    fn Drop(
        &self,
        _pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        _pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> Result<()> {
        // Here we would parse IDataObject for CF_HDROP, CF_UNICODETEXT, CF_DIB etc.
        // For Phase 1 we stub this out since we can't test on Mac.
        // Once parsed, we emit a Tauri event with the payload.
        unsafe {
            *pdweffect = DROPEFFECT_COPY;
        }
        Ok(())
    }
}
