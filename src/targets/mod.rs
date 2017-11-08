//! Contains logger targets and the target trait for implementing custom
//! targets

mod console;
mod dump;
mod escape_chars;
mod json_file;
#[macro_use]
mod target_set;
mod target;

pub use self::console::{Console, ConsoleOptions};
pub use self::dump::Dump;
pub use self::escape_chars::escape_chars;
pub use self::json_file::JsonFile;
pub use self::target_set::TargetSet;
pub use self::target::Target;
