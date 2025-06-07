#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
pub mod client;
pub mod instruction;
pub mod processor;
pub mod state;

// Re-export for convenience
pub use instruction::FenerbahceInstruction;
pub use processor::Processor;
pub use state::{FenerbahceTracker, SeasonData};