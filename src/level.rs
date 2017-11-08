use std::fmt;

/// Log levels to be used with a [`Logger`](struct.Level.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  Error,
  Warn,
  Info,
  Verbose,
  Debug,
  Trace,
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Level::*;
    match self {
      &Error => write!(f, "error"),
      &Warn => write!(f, "warn"),
      &Info => write!(f, "info"),
      &Verbose => write!(f, "verbose"),
      &Debug => write!(f, "debug"),
      &Trace => write!(f, "trace"),
    }
  }
}

impl<S> From<S> for Level
where
  S: AsRef<str>,
{
  fn from(string: S) -> Self {
    use self::Level::*;
    let s = string.as_ref();
    match s {
      "error" => Error,
      "warn" => Warn,
      "info" => Info,
      "verbose" => Verbose,
      "debug" => Debug,
      "trace" => Trace,
      _ => panic!("{} is not a valid level", s),
    }
  }
}
