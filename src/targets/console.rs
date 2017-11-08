use colored::*;
use chrono::Local;
use super::super::Context;
use super::super::Level;
use super::Target;

pub struct Console;

/// Console target
impl Console {
  pub fn new() -> Self {
    Self {}
  }
}

impl Target for Console {
  fn log(&mut self, level: Level, message: &str, context: &Context) {
    let mut message = message.to_string();

    let mut context_pairs: Vec<_> = context.iter().collect();
    context_pairs.sort_by(|a, b| a.0.cmp(b.0));

    message += " ";
    message += &context_pairs
      .into_iter()
      .map(|(key, val)| format!("{}={}", key, val))
      .collect::<Vec<String>>()
      .join(" ");

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

    let timestamp = Local::now().to_rfc2822();

    println!("{} - {} {}", timestamp, level_str, message);
  }
}
