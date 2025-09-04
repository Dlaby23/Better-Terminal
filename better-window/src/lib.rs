// Better Window - Cross-platform window management for Better Terminal
// This is a minimal version - more features will be added incrementally

// Re-export raw-window-handle for compatibility
pub use raw_window_handle;

// Placeholder structures for now  
pub struct Window {
    // TODO: Implement window management
}

pub struct EventLoop {
    // TODO: Implement event loop
}

impl Window {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {}
    }
}

impl EventLoop {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self {}
    }
}
