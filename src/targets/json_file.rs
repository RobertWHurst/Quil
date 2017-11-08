use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;
use super::super::Context;
use super::super::Level;
use super::Target;
use super::escape_chars;

/// Line separated JSON log file target
pub struct JsonFile {
  file: File,
}

impl JsonFile {
  pub fn open<P>(path: P) -> Result<Self, Error>
  where
    P: Into<PathBuf>,
  {
    Ok(Self {
      file: OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.into())?,
    })
  }
}

impl Target for JsonFile {
  fn log(&mut self, level: Level, message: &str, context: &Context) {
    let message = escape_chars(&message, "\"\\");
    let context = context
      .iter()
      .map(|(key, val)| {
        let key = escape_chars(&key, "\"\\");
        let val = escape_chars(&val, "\"\\");
        format!("\"{}\": \"{}\"", key, val)
      })
      .collect::<Vec<String>>()
      .join(", ");

    write!(
      self.file,
      "{{ \"level\": \"{}\", \"message\": \"{}\", \"context\": {{ {} }} }}\n",
      level,
      message,
      context,
    ).unwrap();
  }
}
