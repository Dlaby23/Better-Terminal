use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    println!("Better Terminal v0.1.0");
    println!("This is a minimal terminal emulator based on Rio terminal.");
    println!("More features will be added incrementally.");
    
    // For now, just exit successfully to prove the workspace compiles
    println!("Terminal setup would go here...");
    
    Ok(())
}