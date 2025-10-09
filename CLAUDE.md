# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust Slint GUI framework template project that supports cross-platform development for Desktop (Windows, Linux, macOS), Android, and Web platforms. It includes frequently-used UI components, settings panel, configuration management, database features, and other utilities.

## Build Commands

### Development
- `make desktop-debug` - Run desktop application in debug mode
- `make desktop-debug-winit` - Run with winit backend for fuzzy font resolution on Windows
- `make android-debug` - Run on Android device/emulator
- `make web-debug` - Build and serve web version locally

### Release Builds
- `make desktop-build-release` - Build desktop release binary
- `make android-build-release` - Build Android APK
- `make web-build-release` - Build web distribution

### Testing and Quality
- `make test` - Run tests
- `make clippy` - Run clippy linter
- `make check` - Run cargo check

### Utilities
- `make tr` - Run translation helper
- `make icon` - Generate icons from images
- `make slint-viewer-desktop/android/web` - Preview UI files with auto-reload

## Architecture

### Workspace Structure
- `slint-template/` - Main application crate
- `lib/cutil/` - Utility library (crypto, filesystem, time, strings, etc.)
- `lib/sqldb/` - SQL database abstraction
- `lib/pmacro/` - Procedural macros
- `tr-helper/` - Translation helper tool
- `icon-helper/` - Icon generation tool

### Platform Features
- **Desktop**: Uses `desktop` feature with clipboard, file dialogs, platform directories
- **Android**: Uses `mobile` feature with Android-specific dependencies and permissions
- **Web**: Uses `web` feature with WASM compilation and browser APIs

### UI Architecture
- **Slint UI Files**: Located in `slint-template/ui/`
  - `store.slint` - Global state management and data structures
  - `logic.slint` - UI logic callbacks and functions
  - Platform-specific windows: `desktop-window.slint`, `android-window.slint`, `web-window.slint`
  - Component library in `base/` directory

### Rust Logic
- **Main Entry Points**: Platform-specific main functions in `src/lib.rs`
- **Logic Modules**: Located in `src/logic/` directory
- **Macros**: 
  - `global_store!`, `global_logic!`, `global_util!` - Access global UI objects
  - `logic_cb!` - Connect Slint callbacks to Rust functions
  - `impl_slint_enum_serde!` - Generate serde implementations for Slint enums

### Configuration
- Platform-specific configuration in `src/config.rs`
- Database initialization when `database` feature is enabled
- Automatic logger setup for each platform

## Development Notes

### Platform Requirements
- **Android**: Requires `cargo-apk`, Android SDK/NDK, JDK 17
- **Web**: Requires `wasm-pack`
- **Desktop**: Standard Rust toolchain

### Feature Flags
- `desktop` - Desktop platform support
- `mobile` - Android platform support (includes `android`)
- `web` - Web platform support
- `database` - SQL database support
- `qrcode` - QR code generation
- `center-window` - Window centering utilities

### UI Component Organization
- Components are organized by category in sidebar entries
- Each component has its own `.slint` file in `ui/base/`
- Example implementations in platform-specific panels
- Settings panel with detailed configuration options

### Asset Management
- Icons: `ui/images/icons/` (SVG format)
- Fonts: `ui/fonts/` (Chinese fonts included)
- Images: `ui/images/png/` and `ui/images/crypto-currency/`
- Android resources: `android/res/`