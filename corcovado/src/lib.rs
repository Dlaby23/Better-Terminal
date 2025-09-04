// Corcovado - Non-blocking IO library for Better Terminal
// This is a minimal version - more features will be added incrementally

// Placeholder structures for now
pub struct Poll {
    // TODO: Implement async polling
}

pub struct Token(pub usize);

impl Poll {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl Default for Poll {
    fn default() -> Self {
        Self {}
    }
}
