use std::fmt;
use context::Context;
use targets::{Target, TargetSet};

#[derive(Debug)]
pub struct Logger {
    targets: TargetSet,
    context: Context,
}

impl Logger {
    pub fn new<TS, LC>(targets: TS, context: LC) -> Self
    where
        TS: Into<TargetSet>,
        LC: Into<Context>,
    {
        let targets = targets.into();
        let context = context.into();
        Self { targets, context }
    }

    pub fn child<LC>(&self, context: LC) -> Self
    where
        LC: Into<Context>,
    {
        let context = self.context.merge(context);
        Self {
            targets: self.targets.clone(),
            context,
        }
    }

    pub fn log<S>(&mut self, level: Level, message: S)
    where
        S: Into<String>,
    {
        self.targets.log(level, message.into(), &self.context);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<S> From<S> for Level where S: AsRef<str> {
    fn from(string: S) -> Self {
        use self::Level::*;
        match string.as_ref() {
            "error" => Error,
            "warn" => Warn,
            "info" => Info,
            "verbose" => Verbose,
            "debug" => Debug,
            "trace" => Trace,
            _ => Info
        }
    }
}
