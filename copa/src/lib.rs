// Copa - Configuration parsing for Better Terminal
// This is a minimal version - more features will be added incrementally

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // TODO: Add configuration fields
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
