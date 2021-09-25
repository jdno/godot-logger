//! A simple logger that prints to Godot's output window
//!
//! `godot-logger` is a simple logger that prints log messages to the output console inside the
//! [Godot] game engine. It is built around the logging facade of the [`log`] crate, and uses the
//! [`godot_print!`] macro from the [`gdnative`] bindings.
//!
//! [`gdnative`]: https://crates.io/crates/gdnative
//! [`godot_print!`]: https://docs.rs/gdnative/latest/gdnative/macro.godot_print.html
//! [`log`]: https://crates.io/crates/log
//! [Godot]: https://godotengine.org/

use chrono::Local;
use gdnative::godot_print;
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
        godot_print!(
            "{} {} {}",
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}
