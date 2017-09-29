extern crate scribe;
use scribe::*;

fn main() {
    let mut logger = Logger::new(Console::new(), vec![("context", "example")]);

    logger.log(Level::Error, "foo bar baz");

    let mut sub_logger = logger.child(vec![("context", "sub-example")]);

    sub_logger.log(Level::Info, "test");
}
