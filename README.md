# godot-logger

[![Crates.io](https://img.shields.io/crates/v/godot-logger)](https://crates.io/crates/godot-logger)
[![docs.rs](https://img.shields.io/docsrs/godot-logger)](https://docs.rs/godot-logger)
[![GitHub branch checks state](https://img.shields.io/github/checks-status/jdno/godot-logger/main)](https://github.com/jdno/godot-logger/actions)

_A logger for [godot-rust] projects_

`godot-logger` is an easy-to-use logger for [godot-rust] projects. It prints
logs to Godot's output console and supports module-specific log levels.

## Usage

Start by adding [`godot-logger`] and [`log`] as dependencies to your project's
`Cargo.toml`.

```toml
[dependencies]
godot-logger = "1.1.0"
log = "0.4"
```

Then configure and initialize the logger in the `init` method that is passed to
`godot_init!`.

```rust
use gdnative::prelude::*;
use godot_logger::GodotLogger;
use log::{Level, LevelFilter};

fn init(handle: InitHandle) {
    if let Err(error) = GodotLogger::builder()
        .default_log_level(Level::Info)
        .add_filter("godot_logger", LevelFilter::Debug)
        .init()
    {
        godot_warn!("{}", error.to_string());
    }

    log::debug!("Initialized the logger");
}

godot_init!(init);
```

The following will be printed in the _Output_ console inside Godot:

```text
2021-09-25 19:29:25 DEBUG godot_logger Initialized the logger
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`godot-logger`]: https://crates.io/crates/godot-logger
[`godot_print!`]: https://docs.rs/gdnative/latest/gdnative/macro.godot_print.html
[godot-rust]: https://godot-rust.github.io
[`log`]: https://crates.io/crates/log
