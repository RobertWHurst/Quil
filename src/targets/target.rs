use super::super::Context;
use super::super::Level;

/// Structs that implement `Target` can be used with
/// [`Logger`](struct.Logger.html).
///
/// # Examples
///
/// An example of a basic target that prints to the console:
///
/// ```rust
/// # use quil::prelude::*;
/// # use quil::Level;
/// # use quil::targets::Target;
/// struct Print;
///
/// impl Target for Print {
///   fn log(&mut self, level: Level, message: &str, context: &Context) {
///     println!("LEVEL: {}, MSG: {}, CTX: {}", level, message, context);
///   }
/// }
/// ```
pub trait Target {
  /// You shouldn't need to call `log` directly. `log` is called by logger
  /// internally.
  ///
  /// Log a message to each target contained within the `TargetSet`.
  ///
  /// # Arguments
  ///
  /// * `level` - The log level the message is associated with.
  /// * `message` - The message to log.
  /// * `context` - The context containing meta data associated with the
  ///               message.
  fn log(&mut self, level: Level, message: &str, context: &Context);
}
