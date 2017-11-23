Quil is a easy to use library that supports message levels, multiple
output targets, sub-loggers, and message contexts. Quil can be used across
threads safely, and is easy to extend.

# Examples

Basic example:

```rust
let logger = Logger::new(Console::new(), context!{ src: "root" });

logger.info("hello world");
```

```shell
Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello world src=root
```

Sub-context example:

```rust
let logger = Logger::new(Console::new(), context!{ src: "root", tag: "1" });
logger.info("hello");

let sub_logger = logger.ctx(context!{ tag: "", marker: "49" });
logger.info("world");
```

```shell
Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello src=root tag=1
Tue,  7 Nov 2017 23:55:42 +0000 - info:    world src=root marker=49
```

Multi-target example:

```rust
let logger = Logger::new(targets![
  Console::new(),
  JsonFile::open("path/to/logfile.json"),
], context!{ some_meta_key: "some_meta_value" });

logger.info("hello");

let sub_logger = logger.ctx(context!{ marker: "49" });
logger.warn("world");
```

_Shell_:

```shell
Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello some_meta_key=some_meta_value
Tue,  7 Nov 2017 23:55:42 +0000 - warn:    world some_meta_key=some_meta_value marker=49
```
_Log File_:

```json
{ "level": "info", "message": "hello", "context": { "some_meta_key": "some_meta_value" } }
{ "level": "warn", "message": "world", "context": { "some_meta_key": "some_meta_value", "marker": "49" } }
```

Quil is a easy to use library that supports message levels, multiple
targets, and message contexts. Quil is thread safe and so it is possible
to send loggers accross threads.

[Documentation](https://docs.rs/quil)

# Examples

Basic example:

```rust
let logger = Logger::new(Console::new(), context!{ src: "root" });

logger.info("hello world");
```

```shell
Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello world src=root
```

Sub-context example:

```rust
let logger = Logger::new(Console::new(), context!{ src: "root", tag: "1" });
logger.info("hello");

let sub_logger = logger.ctx(context!{ tag: "", marker: "49" });
logger.info("world");
```

```shell
Tue,  7 Nov 2017 23:55:42 +0000 - info:    hello src=root tag=1
Tue,  7 Nov 2017 23:55:42 +0000 - info:    world src=root marker=49
```