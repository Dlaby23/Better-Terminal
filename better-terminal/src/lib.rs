// Better Terminal Library
// This is a minimal version - more features will be added incrementally

pub use better_gpu;
pub use better_pty;
pub use better_window;
pub use copa;

// Re-export common items
pub mod config {
    pub use copa::Config;
}

pub mod terminal {
    pub struct Terminal {
        // TODO: Implement terminal functionality
    }
    
    impl Terminal {
        pub fn new() -> Self {
            Self {}
        }
    }
}
