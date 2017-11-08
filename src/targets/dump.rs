use super::super::Context;
use super::super::Level;
use super::Target;

pub struct Dump {
  messages: Vec<(Level, String, Context)>,
}

impl Dump {
  pub fn new() -> Self {
    Self {
      messages: Vec::new(),
    }
  }

  pub fn dump_messages(&mut self) -> Vec<(Level, String, Context)> {
    self.messages.drain(..).collect()
  }
}

impl Target for Dump {
  fn log(&mut self, level: Level, message: &str, context: &Context) {
    self
      .messages
      .push((level, message.to_string(), context.clone()));
  }
}
