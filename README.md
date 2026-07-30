# Stash

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-v2-orange.svg)](https://v2.tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-v5-ff3e00.svg)](https://svelte.dev/)
[![Build Status](https://github.com/NaveLIL/Stash/actions/workflows/windows-build.yml/badge.svg)](https://github.com/NaveLIL/Stash/actions)

**Stash** is a lightweight, visually stunning Drop Zone utility designed exclusively for Windows. It provides a temporary visual buffer (a "shelf") for files, text, and images you want to hold onto for a short time while moving between applications.

Built with performance and aesthetics in mind, using **Tauri v2 (Rust)**, **Svelte 5**, and **TailwindCSS**.

## 🚀 Features

- **Global Accessibility:** Summon Stash instantly from anywhere using the `Ctrl+Shift+Space` global hotkey.
- **Edge Magnetism:** The window automatically snaps to the right edge of your current monitor.
- **Smart Drag & Drop:** Custom OLE integration allows you to drag files and text *into* Stash, and drag them *out* into any other Windows application.
- **Auto-Cleanup:** Items placed in Stash are strictly temporary. They expire and vanish after 15 minutes to keep your workspace clean.
- **Quick Actions:** Hover over a card to trigger quick transformations:
  - 🗜️ **Compress Image:** Instantly shrink JPG/PNG files using the Rust `image` backend.
  - 📦 **Zip File:** Create a `.zip` archive from dropped files.
  - ✂️ **Clean URL:** Automatically strips tracking parameters (like `utm_source`) from copied links.
  - 📱 **Generate QR Code:** Instantly generate a QR code for any URL to quickly scan with your phone.
- **P2P Local Share:** Stash instances automatically discover each other on the local network (via mDNS). Securely send files across devices with a one-time 4-digit PIN!
- **Multi-language Support:** UI automatically translates to English, Russian, Spanish, French, German, Chinese, and Japanese based on your system language.

## 🛠️ Development Setup

You will need [Node.js](https://nodejs.org/) (v20+) and [Rust](https://rustup.rs/) installed on your machine.

1. Clone the repository:
   ```bash
   git clone https://github.com/NaveLIL/Stash.git
   cd Stash
   ```
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

## 🧪 Testing

Stash includes automated tests covering both the Svelte logic and Rust utilities.

Run Svelte UI tests (Vitest):
```bash
npm run test
```

Run Rust backend tests:
```bash
cargo test --workspace --manifest-path src-tauri/Cargo.toml
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
Copyright (c) 2026 NaveLIL. All rights reserved.
