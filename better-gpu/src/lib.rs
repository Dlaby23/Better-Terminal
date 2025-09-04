// Better GPU - Rendering engine for Better Terminal
// This is a minimal version - more features will be added incrementally

// Re-export WGPU
pub use wgpu;

// Placeholder structures for now
pub struct BetterRenderer {
    // TODO: Implement rendering
}

impl BetterRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BetterRenderer {
    fn default() -> Self {
        Self::new()
    }
}