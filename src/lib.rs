extern crate colored;
extern crate chrono;

mod context;
mod logger;
mod targets;

pub use logger::{Logger, Level};
pub use context::Context;
pub use targets::{Target, TargetSet, Console, LogDebug};
