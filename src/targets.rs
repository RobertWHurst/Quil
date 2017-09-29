use colored::*;
use chrono::Utc;
use std::fmt;
use std::sync::{Arc, Mutex};
use context::Context;
use logger::Level;

pub trait Target: fmt::Debug {
    fn log(&mut self, level: Level, message: String, context: &Context);
}

#[derive(Debug, Clone)]
pub struct TargetSet {
    targets: Arc<Mutex<Vec<Box<Target>>>>,
}

impl TargetSet {
    pub fn from_target<T>(target: T) -> Self
    where
        T: Target + 'static,
    {
        Self { targets: Arc::new(Mutex::new(vec![Box::new(target)])) }
    }
}

impl Target for TargetSet {
    fn log(&mut self, level: Level, message: String, context: &Context) {
        let mut targets = self.targets.lock().unwrap();
        for target in targets.iter_mut() {
            target.log(level, message.clone(), context);
        }
    }
}

#[derive(Debug)]
pub struct Console;

impl Console {
    pub fn new() -> Self {
        Self {}
    }
}

impl Target for Console {
    fn log(&mut self, level: Level, mut message: String, context: &Context) {
        for (key, val) in context.iter() {
            message += &format!(" {}={}", key, val);
        }

        let level_str = level.to_string();
        let pad       = " ".repeat(7 - level_str.chars().count());

        let mut level_str = match level {
            Level::Error => format!("{}", level_str).red().to_string(),
            Level::Warn => format!("{}", level_str).yellow().to_string(),
            Level::Info => format!("{}", level_str).white().to_string(),
            Level::Verbose => format!("{}", level_str).cyan().to_string(),
            Level::Debug => format!("{}", level_str).white().dimmed().to_string(),
            Level::Trace => format!("{}", level_str).white().dimmed().to_string(),
        };

        level_str += ":";
        level_str += &pad;

        let timestamp = Utc::now().to_rfc2822();

        println!("{} - {} {}", timestamp, level_str, message);
    }
}

impl From<Console> for TargetSet {
    fn from(console_target: Console) -> Self {
        Self::from_target(console_target)
    }
}

#[derive(Debug, Clone)]
pub struct LogDebug {
    messages: Arc<Mutex<Vec<(Level, String, Context)>>>,
}

impl LogDebug {
    pub fn new() -> Self {
        Self { messages: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn dump_messages(&mut self) -> Vec<(Level, String, Context)> {
        let mut messages = self.messages.lock().unwrap();
        let messages = messages.drain(..).collect();
        messages
    }
}

impl Target for LogDebug {
    fn log(&mut self, level: Level, message: String, context: &Context) {
        self.messages.lock().unwrap().push((
            level,
            message,
            context.clone(),
        ));
    }
}

impl From<LogDebug> for TargetSet {
    fn from(test_target: LogDebug) -> Self {
        Self::from_target(test_target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::Context;

    #[test]
    fn can_create_target_set_from_target() {
        let _: TargetSet = TargetSet::from_target(LogDebug::new());
    }

    #[test]
    fn calling_log_on_target_set_calls_log_on_each_target() {
        let mut target = LogDebug::new();
        let mut target_set = TargetSet::from_target(target.clone());

        target_set.log(
            Level::Info,
            "test message".to_string(),
            &Context::from(vec![("key1", "val1"), ("key2", "val2"), ("key3", "val3")]),
        );

        let messages = target.dump_messages();
        assert_eq!(messages.len(), 1);

        let message = messages.first().unwrap();
        assert_eq!(message.0, Level::Info);
        assert_eq!(message.1, "test message");
        assert_eq!(message.2.data.get("key1").unwrap(), "val1");
        assert_eq!(message.2.data.get("key2").unwrap(), "val2");
        assert_eq!(message.2.data.get("key3").unwrap(), "val3");
    }
}
