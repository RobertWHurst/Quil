#[macro_use(targets, context)]
extern crate quil;
use quil::prelude::*;

fn main() {
  // Create a new logger that logs to the console and writes to a file
  let logger = Logger::new(
    targets![
      Console::new(),
      JsonFile::open("examples/basic.json").unwrap()
    ],
    context!{ src: "\"example" },
  );

  // log hellow world
  logger.info("hello world");

  // create a sub logger
  let sub_logger = logger.ctx(context!{
    sub_src: "subspace"
  });

  // log beep boop
  sub_logger.verbose("beep \"\\ boop");

  // log beep beep to a sub sub logger
  sub_logger.ctx(context!{ sub_src: "" }).warn("beep beep");
}
