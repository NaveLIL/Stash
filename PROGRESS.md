# Progress

## Completed
- **Phase 0:** Tauri v2 setup, TailwindCSS + Svelte 5 runes configured, transparent/vibrant background setup.
- **Phase 1:** Custom Windows OLE `IDropTarget` implemented in Rust (`drop_target.rs`). Drag-IN working via custom OLE messages. Drag-OUT working via `@crabnebula/tauri-plugin-drag`.
- **Phase 2:** Centralized state using Svelte 5 `store.svelte.ts`. Card UI (`Card.svelte`) and layout (`CardList.svelte`) with `svelte/motion` springs.
- **Phase 3:** Global hotkey `Ctrl+Shift+Space` toggles window visibility (`tauri-plugin-global-shortcut`). Edge magnetism snapping. Auto-cleanup TTL timer (15 min) in store.
- **Phase 4:** Quick actions in Rust (`actions.rs`) for image compression, ZIP creation, URL cleaning, and QR code generation. Hover UI inside `Card.svelte`.
- **Phase 5:** P2P file sharing (`p2p.rs`). Uses `mdns-sd` for discovery and an `axum` HTTP server for receiving files. Authenticated via a randomly generated 4-digit PIN.

## Next Steps
- **Windows Verification:** Wait for the GitHub Action to compile the `msi` installer. The USER must download it, install it on a Windows machine, and test the full flow (Drag and Drop, Global Shortcuts, Quick Actions, P2P).

## Known Issues
- Currently awaiting manual testing on a real Windows machine to ensure OLE drag-and-drop parsing logic correctly captures complex Explorer memory formats.
