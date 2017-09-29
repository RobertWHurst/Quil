extern crate scribe;

use scribe::*;

#[test]
fn can_create_console_logger() {
    let _: Logger = Logger::new(
        LogDebug::new(),
        vec![("key1", "val1"), ("key2", "val2"), ("key3", "val3")],
    );
}

#[test]
fn can_log_a_message() {
    let mut target = LogDebug::new();
    let mut logger = Logger::new(
        target.clone(),
        vec![("key1", "val1"), ("key2", "val2"), ("key3", "val3")],
    );

    logger.log(Level::Info, "test message");

    let messages = target.dump_messages();
    assert_eq!(messages.len(), 1);

    let message = messages.first().unwrap();
    assert_eq!(message.0, Level::Info);
    assert_eq!(message.1, "test message");
    assert_eq!(message.2.data.get("key1").unwrap(), "val1");
    assert_eq!(message.2.data.get("key2").unwrap(), "val2");
    assert_eq!(message.2.data.get("key3").unwrap(), "val3");
}
