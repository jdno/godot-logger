# godot-logger

_Logger for [godot-rust] projects_

`godot-logger` is a logger implementation for [godot-rust] projects that prints
logs using [`godot_print!`].

## Usage

Add [`godot-logger`] and [`log`] as dependencies to `Cargo.toml`.

Then initialize `godot-logger` in the `init` function that is exported by
`gdnative`

```rust
use gdnative::prelude::*;

fn init(handle: InitHandle) {
    godot_logger::init(Level::Debug);
    log::debug!("Initialized the logger");
}

godot_init!(init);
```

The following will be printed in the _Output_ console inside Godot:

```text
2021-09-25 19:29:25 DEBUG Initialized the logger
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
