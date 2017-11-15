use std::sync::{Arc, Mutex, RwLock};
use super::Context;
use super::Target;
use super::Level;

/// Logger has methods for logging messages from your program. This struct is
/// the heart of Quil. Logger is clonable and can be send across threads safely.
///
/// # Examples
///
/// Create a basic logger and log a message:
///
/// ```rust
/// # #[macro_use] extern crate quil;
/// # fn main() {
/// # use quil::prelude::*;
/// let logger = Logger::new(Console::new(), context!{});
/// logger.info("Hello world");
/// # }
/// ```
#[derive(Clone)]
pub struct Logger {
  target: Arc<Mutex<Box<Target + 'static>>>,
  level: Arc<RwLock<Level>>,
  context: Arc<Context>,
}

impl Logger {
  /// Creates a new logger from a target and context.
  ///
  /// The target can be any type that implements the
  /// [`Target`](trait.Target.html) trait. If you wish to use more than one
  /// target with your logger It's recomended you use the
  /// [`TargetSet`](struct.TargetSet.html) via the
  /// [`targets!`](macro.targets.html) macro. The
  /// [`Context`](struct.Context.html) contains any contextual meta data
  /// you want to pass to the target with each log message. It's recommended
  /// you use the [`context`](macro.ctx.html) macro to create your
  /// [`Context`](struct.Context.html).
  ///
  /// See [`Logger` examples](struct.Logger.html#examples) above.
  ///
  /// # Arguments
  /// * `target` - The target the logger will log to.
  /// * `context` - The context that will be passed with each log message.
  ///             to the target.
  pub fn new<T>(target: T, context: Context) -> Self
  where
    T: Target + 'static,
  {
    Self {
      target: Arc::new(Mutex::new(Box::new(target))),
      level: Arc::new(RwLock::new(Level::Trace)),
      context: Arc::new(context),
    }
  }

  /// Changes the logger's loggin level.
  ///
  /// Note that this will change the level for all loggers that share the same
  /// ancestor.
  ///
  /// # Arguments
  ///
  /// * `level` - The log level you'd like to set.
  pub fn set_level<L>(&self, level: L)
  where
    L: Into<Level>,
  {
    *self.level.write().unwrap() = level.into();
  }

  /// Changes the logger target.
  ///
  /// Note that this will change the target for all loggers that share the same
  /// ancestor.
  ///
  /// # Arguments
  ///
  /// * `target` - The target you'd like the logger to use.
  pub fn set_target<T>(&self, target: T)
  where
    T: Target + 'static,
  {
    *self.target.lock().unwrap() = Box::new(target);
  }

  /// Creates a new logger with an extended context.
  ///
  /// The returned logger with have a context containing all values from
  /// the logger `ctx` is called upon and the given context. Keys that
  /// are present in both contexts will be overwritten by the given context.
  ///
  /// If you want to remove a key that was set in the original logger set the
  /// value of the key to an empty string literal. This will remove it from
  /// the new logger.
  ///
  /// # Arguments
  ///
  /// * `context` - A context containing meta data for the new logger.
  ///
  /// # Examples
  ///
  /// Basic example of using `ctx` to create a sub logger:
  ///
  /// ```rust
  /// # #[macro_use] extern crate quil;
  /// # fn main() {
  /// # use quil::prelude::*;
  /// let logger = Logger::new(Console::new(), context!{ src: "root" });
  /// logger.info("hello");
  /// let sub_logger = logger.ctx(context!{ src: "sub_root" });
  /// logger.info("world");
  /// # }
  /// ```
  ///
  /// ```shell
  /// Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello src=root
  /// Tue,  7 Nov 2017 23:55:42 +0000 - info:    world src=sub_root
  /// ```
  ///
  /// Example of removing a key:
  ///
  /// ```rust
  /// # #[macro_use] extern crate quil;
  /// # fn main() {
  /// # use quil::prelude::*;
  /// let logger = Logger::new(Console::new(), context!{ src: "root" });
  /// logger.info("hello");
  /// let sub_logger = logger.ctx(context!{ src: "" });
  /// logger.info("world");
  /// # }
  /// ```
  ///
  /// ```shell
  /// Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello src=root
  /// Tue,  7 Nov 2017 23:55:42 +0000 - info:    world
  /// ```
  pub fn ctx(&self, context: Context) -> Self {
    Self {
      target: self.target.clone(),
      level: self.level.clone(),
      context: Arc::new(self.context.merge(context)),
    }
  }

  /// Log an error message.
  ///
  /// It's recommended to use this method only to log errors.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log.
  pub fn error(&self, message: &str) {
    self.log(Level::Error, message);
  }

  /// Log a warning message.
  ///
  /// It's recommended to use this method only to log warnings.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log.
  pub fn warn(&self, message: &str) {
    self.log(Level::Warn, message);
  }

  /// Log an informational message.
  ///
  /// It's recommended to use this method for messages that should be seen
  /// by operators of your application without debugging enabled.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log
  pub fn info(&self, message: &str) {
    self.log(Level::Info, message);
  }

  /// Log an verbose message.
  ///
  /// It's recommended to use this method for messages that should only be
  /// seen if an operator needs to see a bit more than the info level.
  ///
  /// Note that verbose is not the same as trace. It's intented to allow for
  /// a bit of extra informational detail.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log
  pub fn verbose(&self, message: &str) {
    self.log(Level::Verbose, message);
  }

  /// Log an debug message.
  ///
  /// It's recommended to use this method for messages that should only be
  /// seen if an operator or developer needs a debug information.
  ///
  /// Any messages at this level should be intented to help operators and
  /// developers understand what is happening in your application without
  /// a full on trace.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log
  pub fn debug(&self, message: &str) {
    self.log(Level::Debug, message);
  }

  /// Log an trace message.
  ///
  /// It's recommended to use this method for messages that should only be
  /// seen if an operator or developer needs a trace information.
  ///
  /// Trace should be used to explain to developers and operators what is
  /// happening at a low level within the application.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log
  pub fn trace(&self, message: &str) {
    self.log(Level::Trace, message);
  }

  /// Log a message.
  ///
  /// It's recomended to use one of the level bound methods above instead of
  /// `log`. However if you want to select the log level dynamically this
  /// method can be used.
  ///
  /// # Arguments
  ///
  /// * `message` - A pointer the message to log
  pub fn log<L>(&self, level: L, message: &str)
  where
    L: Into<Level>,
  {
    let level = level.into();
    if level > *self.level.read().unwrap() {
      return;
    }
    self
      .target
      .lock()
      .unwrap()
      .log(level, message, &self.context);
  }
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}
