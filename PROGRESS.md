# Stash Project Progress

## Current Status
**Phase 0 — Framework:** Completed
**Current Phase:** Phase 1 (Pending)

## Completed Phases

### Phase 0 — Framework
- Initialized Tauri v2 + Svelte 5 + TS + Tailwind in the `stash` directory.
- Configured a transparent, frameless, always-on-top window hidden from the taskbar.
- Implemented a system tray icon with "Show" and "Quit" options.
- Added `window-vibrancy` for native window blur effects (Acrylic/Mica on Windows, HudWindow on macOS).
- **Known Issues:** None so far.

## Next Up
### Phase 1 — Core DnD (Critical Risk)
- Implement Drag-OUT using `tauri-plugin-drag`.
- Implement Drag-IN using `windows-rs` (OLE IDropTarget).
- Verify end-to-end drag and drop functionality.
