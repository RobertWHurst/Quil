#[macro_use(targets, context)]
extern crate quil;
use quil::prelude::*;

fn main() {
  let logger = Logger::new(
    targets![
      Console::new(),
      JsonFile::open("examples/basic.log").unwrap()
    ],
    context!{ src: "\"example" },
  );
  logger.info("hello world");

  let sub_logger = logger.ctx(context!{
    sub_src: "subspace"
  });
  sub_logger.verbose("beep \"\\ boop");

  sub_logger.ctx(context!{ sub_src: "" }).warn("beep beep");
}
