# Better Terminal Build System

.PHONY: all build run clean test dev release

all: build

# Development build
dev:
	cargo build
	./target/debug/better

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run the terminal
run:
	cargo run --release

# Run with debug logging
debug:
	RUST_LOG=debug cargo run

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Check code
check:
	cargo check --all-targets

# Lint code
lint:
	cargo clippy -- -D warnings

# Install locally
install: release
	cp target/release/better /usr/local/bin/

# Uninstall
uninstall:
	rm -f /usr/local/bin/better

# Build for macOS
macos: release
	@echo "Built for macOS at target/release/better"

# Build for Linux
linux:
	cargo build --release --target x86_64-unknown-linux-gnu

# Build for Windows
windows:
	cargo build --release --target x86_64-pc-windows-gnu

help:
	@echo "Better Terminal Build Commands:"
	@echo "  make dev      - Build and run in development mode"
	@echo "  make build    - Build in debug mode"
	@echo "  make release  - Build in release mode"
	@echo "  make run      - Run the terminal"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make test     - Run tests"
	@echo "  make fmt      - Format code"
	@echo "  make lint     - Lint code"
	@echo "  make install  - Install to /usr/local/bin"