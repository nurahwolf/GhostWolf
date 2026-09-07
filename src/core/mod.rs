mod config;
mod context;
mod emoji;
mod guild;
mod resume;
mod traits;
mod users;

pub use config::*;
pub use context::*;
pub use emoji::*;
pub use guild::*;
pub use resume::*;
pub use traits::*;
pub use users::*;

/// A global counter for how many times the bot has been booped (since restart).
pub static BOOP_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
