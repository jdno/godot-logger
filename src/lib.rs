//! A simple logger that prints to Godot's output window
//!
//! `godot-logger` is a simple logger that prints log messages to the output console inside the
//! [Godot] game engine. It is built around the logging facade of the [`log`] crate, and uses the
//! [`godot_print!`] macro from the [`gdnative`] bindings.
//!
//! # Use
//!
//! Add [`godot-logger`] and [`log`] as dependencies to `Cargo.toml`.
//!
//! Then initialize `godot-logger` in the `init` function that is exported by `gdnative`.
//!
//! ```no_run
//! use gdnative::prelude::*;
//! use log::Level;
//!
//! fn init(handle: InitHandle) {
//!     godot_logger::init(Level::Debug);
//!     log::debug!("Initialized the logger");
//! }
//!
//! godot_init!(init);
//! ```
//!
//! The following will appear in the _Output_ console inside Godot:
//!
//! ```text
//! 2021-09-25 19:29:25 DEBUG Initialized the logger
//! ```
//!
//! [`gdnative`]: https://crates.io/crates/gdnative
//! [`godot-logger`]: https://crates.io/crates/godot-logger
//! [`godot_print!`]: https://docs.rs/gdnative/latest/gdnative/macro.godot_print.html
//! [`log`]: https://crates.io/crates/log
//! [Godot]: https://godotengine.org/

use chrono::Local;
use gdnative_core::{godot_print, godot_warn};
use log::{Level, Log, Metadata, Record, SetLoggerError};

static LOGGER: GodotLogger = GodotLogger;

/// Initialize the logger
///
/// The logger is initialized with a maximum [`Level`][log::Level]. Any log message up to this log
/// level will be printed to Godot's _Output_ window with the current time and its severity.
pub fn init(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level.to_level_filter()))
}

struct GodotLogger;

impl Log for GodotLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        let message = format!(
            "{} {} {}",
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            record.level(),
            record.args()
        );

        if record.level() <= Level::Warn {
            godot_warn!("{}", message);
        } else {
            godot_print!("{}", message);
        }
    }

    fn flush(&self) {}
}
