# Better Terminal

A terminal emulator based on the Rio terminal source code, redesigned with a modular architecture.

## Architecture

Better Terminal is built with four core modules:

- **better-terminal**: Main application and entry point
- **better-gpu**: Hardware-accelerated rendering engine (based on Sugarloaf)
- **better-pty**: Terminal emulation and PTY handling (based on Teletypewriter)
- **better-window**: Cross-platform window management (based on Rio-Window)

Additional supporting modules:
- **corcovado**: Non-blocking IO library
- **copa**: Configuration parsing

## Current Status

This is a minimal but functional implementation that compiles and runs. All Rio references have been renamed to "better" equivalents.

### What's Working

- ✅ Rust workspace compiles successfully
- ✅ All modules build without errors
- ✅ Basic binary executes and displays version info
- ✅ Modular architecture in place

### What's Coming

- Terminal window creation
- Text rendering
- PTY integration
- Input handling
- Configuration system

## Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run
cargo run --bin better
# or
./target/release/better
```

## Module Structure

```
better-terminal/
├── Cargo.toml              # Workspace configuration
├── better-terminal/        # Main application
├── better-gpu/             # Rendering engine  
├── better-pty/             # Terminal emulation
├── better-window/          # Window management
├── corcovado/              # Async I/O
└── copa/                   # Configuration
```

## Dependencies

The project uses minimal dependencies for the current implementation:
- wgpu for GPU rendering
- tracing for logging  
- serde for serialization
- Platform-specific windowing libraries

## License

MIT License (inherited from Rio terminal)