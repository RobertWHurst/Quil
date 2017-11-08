use colored::*;
use chrono::Local;
use super::super::Context;
use super::super::Level;
use super::Target;

pub struct ConsoleOptions {
  show_timestamp: bool,
  show_level: bool,
  show_context: bool,
}

impl ConsoleOptions {
  pub fn new() -> Self {
    Self {
      show_timestamp: false,
      show_level: false,
      show_context: false,
    }
  }

  pub fn show_timestamp(mut self, show_timestamp: bool) -> Self {
    self.show_timestamp = show_timestamp;
    self
  }

  pub fn show_level(mut self, show_level: bool) -> Self {
    self.show_level = show_level;
    self
  }

  pub fn show_context(mut self, show_context: bool) -> Self {
    self.show_context = show_context;
    self
  }

  pub fn build(self) -> Console {
    Console {
      show_timestamp: self.show_timestamp,
      show_level: self.show_level,
      show_context: self.show_context,
    }
  }
}

pub struct Console {
  show_timestamp: bool,
  show_level: bool,
  show_context: bool,
}

/// Console target
impl Console {
  pub fn new() -> Self {
    Self {
      show_timestamp: true,
      show_level: true,
      show_context: true,
    }
  }
}

impl Target for Console {
  fn log(&mut self, level: Level, message: &str, context: &Context) {
    let mut message = message.to_string();

    let mut context_pairs: Vec<_> = context.iter().collect();
    context_pairs.sort_by(|a, b| a.0.cmp(b.0));

    if self.show_context {
      message += " ";
      message += &context_pairs
        .into_iter()
        .map(|(key, val)| format!("{}={}", key, val))
        .collect::<Vec<String>>()
        .join(" ");
    }

    if self.show_level {
      let level_str = level.to_string();
      let pad = " ".repeat(7 - level_str.chars().count());

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

      message = format!("{} {}", level_str, message);
    }

    if self.show_timestamp {
      let timestamp = Local::now().to_rfc2822();
      message = format!("{} - {}", timestamp, message);
    }

    println!("{}", message);
  }
}
