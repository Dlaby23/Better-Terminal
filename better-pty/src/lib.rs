// Better PTY - Terminal emulation for Better Terminal
// This is a minimal version - more features will be added incrementally

// Placeholder structures for now
pub struct Pty {
    // TODO: Implement PTY handling
}

impl Pty {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl Default for Pty {
    fn default() -> Self {
        Self {}
    }
}

// Export common types
pub type WinSize = (u16, u16);
