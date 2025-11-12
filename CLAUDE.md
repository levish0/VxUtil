# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VxUtil is a video editor being developed in Rust as an alternative to Adobe Premiere Pro. The project is in early development stages with a focus on high-performance video processing and editing capabilities.

### Workspace Structure

The project is organized as a Cargo workspace with three crates:

```
VxUtil/
├── vxutil-core/      # Business logic and domain models
├── vxutil-engine/    # Media processing and rendering
└── vxutil-ui/        # Iced-based user interface (main binary)
```

**`vxutil-core`**: Pure business logic with NO UI or media processing dependencies
- Project management (`Project`, `ProjectSettings`)
- Timeline data models (sequences, tracks, clips)
- Media library management
- Effects system (traits and parameters)
- Common types (`Timecode`, `FrameRate`, `Resolution`, `TimeRange`)
- Error handling (`VxError`, `Result`)

**`vxutil-engine`**: Media processing layer with NO UI dependencies
- FFmpeg wrapper for video/audio decode/encode
- Real-time playback engine
- Rendering and compositing pipeline
- GPU acceleration via WGPU
- Frame caching system
- Depends on: `vxutil-core`, `ffmpeg-next`, `wgpu`, `rayon`

**`vxutil-ui`**: Iced-based GUI application (produces the `vxutil` binary)
- Main application state and message handling
- UI components (timeline editor, preview window, panels)
- Integrates `vxutil-core` for data models and `vxutil-engine` for playback/rendering
- Depends on: `vxutil-core`, `vxutil-engine`, `iced`

## Building and Running

### System Dependencies (Required)

This project requires FFmpeg libraries. Install them based on your platform:

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y \
  libasound2-dev \
  libavcodec-dev \
  libavformat-dev \
  libavutil-dev \
  libavfilter-dev \
  libavdevice-dev \
  libswscale-dev \
  libswresample-dev \
  pkg-config
```

**macOS:**
```bash
brew install ffmpeg pkg-config
```

**Windows:**
Install FFmpeg from https://ffmpeg.org/download.html and ensure it's in your PATH.

### Build Commands

```bash
# Build all workspace crates
cargo build

# Build specific crate
cargo build -p vxutil-core
cargo build -p vxutil-engine
cargo build -p vxutil-ui

# Release build (optimized)
cargo build --release

# Run the application (vxutil-ui binary)
cargo run -p vxutil-ui

# Run with release optimizations
cargo run -p vxutil-ui --release
```

### Testing

```bash
# Run all tests in workspace
cargo test

# Test specific crate
cargo test -p vxutil-core
cargo test -p vxutil-engine
cargo test -p vxutil-ui

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Check all workspace crates without building
cargo check

# Check specific crate
cargo check -p vxutil-core

# Format all code in workspace
cargo fmt

# Lint with clippy
cargo clippy --workspace

# Clippy with all warnings
cargo clippy --workspace -- -W clippy::all
```

## Architecture

### Core Technology Stack

- **UI Framework**: `iced` (0.13.1) - Cross-platform GUI framework with immediate mode rendering
- **Video Processing**: `ffmpeg-next` (8.0.0) - Rust bindings to FFmpeg for video decode/encode/processing
- **GPU Acceleration**: `wgpu` (27.0.1) - WebGPU API for hardware-accelerated rendering and compute
- **Audio**: `cpal` (0.16.0) - Cross-platform audio I/O
- **Async Runtime**: `tokio` (1.48.0) - For async operations and I/O
- **Concurrency**:
  - `rayon` (1.11.0) - Data parallelism
  - `crossbeam` (0.8.4) - Lock-free concurrent primitives
  - `parking_lot` (0.12.5) - Faster synchronization primitives
- **Logging**: `tracing` ecosystem with JSON output and file appender support

### Architectural Considerations

When developing features, consider:

1. **Performance**: This is a video editor requiring real-time performance
   - Use `rayon` for parallel processing of frames/data
   - Leverage `wgpu` for GPU-accelerated operations
   - Use `parking_lot` for low-overhead synchronization
   - Profile before optimizing, but be mindful of hot paths

2. **Async Operations**:
   - File I/O and network operations should use `tokio`
   - Video processing pipelines may need async/await for streaming
   - Be careful mixing blocking FFmpeg operations with async code

3. **Memory Management**:
   - Video frames are large; be conscious of allocation patterns
   - Consider using object pools for frequently allocated structures
   - Use `image` crate (0.25.8) for efficient image handling

4. **Error Handling**:
   - Use `anyhow` for application-level errors with context
   - Use `thiserror` for library-level typed errors
   - FFmpeg errors should be wrapped with context

5. **UI Architecture** (Iced 0.13+):
   - Uses function-based application builder: `iced::application(title, update, view)`
   - Returns `Task<Message>` instead of `Command<Message>`
   - Keep UI state (`vxutil-ui`) separate from core domain models (`vxutil-core`)
   - Business logic lives in `vxutil-core`, not in UI message handlers

6. **Crate Boundaries**:
   - `vxutil-core`: Never import `iced`, `wgpu`, or `ffmpeg-next`
   - `vxutil-engine`: Never import `iced`
   - `vxutil-ui`: Can import both `vxutil-core` and `vxutil-engine`
   - When adding features, decide which crate owns the logic

### Rust Edition

This project uses Rust Edition 2024 with minimum version 1.91.1. Ensure your toolchain is up to date.

## Development Workflow

The CI pipeline runs release builds on every push/PR to main. Ensure your code:
- Builds successfully with `cargo build --release`
- Has FFmpeg system dependencies available (or CI will fail)
- Follows Rust idioms and conventions