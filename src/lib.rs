//! A simple logger that prints to Godot's output window
//!
//! [godot-logger] is a simple logger that prints log messages to the output console inside the
//! [Godot] game engine. It is built around the logging facade of the [log] crate, and uses the
//! [`godot_print!`] macro from the [gdnative] bindings.
//!
//! It is possible to configure different log levels for different Rust modules, similar to other
//! popular logging frameworks such as [env_logger] or [log4rs]. Simply provide a list as the second
//! argument to the `init` function with module names and log levels.
//!
//! # Use
//!
//! Add [godot-logger] and [log] as dependencies to `Cargo.toml`.
//!
//! Then initialize [godot-logger] in the `init` function that is exported by `gdnative`. Pass in a
//! default log level, and a list with module-level overrides (can be empty).
//!
//! ```no_run
//! use gdnative::prelude::*;
//! use godot_logger::Filter;
//! use log::Level;
//!
//! fn init(handle: InitHandle) {
//!     let filters = vec![Filter::new("h2", Level::Error)];
//!
//!     godot_logger::init(Level::Debug, filters);
//!     log::debug!("Initialized the logger");
//! }
//!
//! godot_init!(init);
//! ```
//!
//! The following will appear in the _Output_ console inside Godot:
//!
//! ```text
//! 2021-09-25 19:29:25 DEBUG godot-logger Initialized the logger
//! ```
//!
//! [env_logger]: https://crates.io/crates/env_logger
//! [gdnative]: https://crates.io/crates/gdnative
//! [godot-logger]: https://crates.io/crates/godot-logger
//! [`godot_print!`]: https://docs.rs/gdnative/latest/gdnative/macro.godot_print.html
//! [log]: https://crates.io/crates/log
//! [log4rs]: https://crates.io/crates/log4rs
//! [Godot]: https://godotengine.org/

use chrono::Local;
use gdnative_core::{godot_print, godot_warn};
use log::{Level, LevelFilter, Record, SetLoggerError};
use log4rs::append::Append;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;

const APPENDER_NAME: &str = "godot-logger";

/// Initialize the logger
///
/// The logger is initialized with a default log level and a list of module-level overrides. The
/// overrides follow the convention of popular logging frameworks such as [env_logger] and [log4rs],
/// and combine a module path with a log level.
///
/// # Example
///
/// ```
/// use gdnative::prelude::*;
/// use godot_logger::Filter;
/// use log::Level;
///
/// fn init(handle: InitHandle) {
///     let filters = vec![Filter::new("h2", Level::Error)];
///
///     godot_logger::init(Level::Debug, filters);
///     log::debug!("Initialized the logger");
/// }
///
/// godot_init!(init);
/// ```
pub fn init(default_level: Level, filters: Vec<Filter>) -> Result<(), SetLoggerError> {
    let loggers: Vec<Logger> = filters
        .iter()
        .map(|filter| {
            Logger::builder()
                .appender(APPENDER_NAME)
                .build(filter.module, filter.level)
        })
        .collect();

    let config = Config::builder()
        .appender(Appender::builder().build(APPENDER_NAME, Box::new(GodotAppender)))
        .loggers(loggers)
        .build(
            Root::builder()
                .appender(APPENDER_NAME)
                .build(default_level.to_level_filter()),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;
    Ok(())
}

/// A filter to apply a custom log level to a Rust module
///
/// Logs in [godot-logger] can be filtered using the default log level or a module-level override.
/// Module-level overrides are configured using a `Filter`, which combines a module path in Rust
/// with a log level.
///
/// # Example
///
/// ```
/// use godot_logger::Filter;
/// use log::Level;
///
/// let filter = Filter::new("godot-logger", Level::Error);
/// ```
pub struct Filter {
    module: &'static str,
    level: LevelFilter,
}

impl Filter {
    /// Initialize a new filter
    ///
    /// Filters combine a module path in Rust with a log level.
    ///
    /// # Example
    ///
    /// ```
    /// use godot_logger::Filter;
    /// use log::Level;
    ///
    /// let filter = Filter::new("godot-logger", Level::Error);
    /// ```
    pub fn new(module: &'static str, level: Level) -> Self {
        Self {
            module,
            level: level.to_level_filter(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct GodotAppender;

impl Append for GodotAppender {
    fn append(&self, record: &Record) -> anyhow::Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let level = record.level();
        let message = record.args();

        let output = match record.module_path() {
            Some(module) => format!("{} {} {} {}", timestamp, level, module, message),
            None => format!("{} {} {}", timestamp, level, message),
        };

        if record.level() <= Level::Warn {
            godot_warn!("{}", output);
        } else {
            godot_print!("{}", output);
        }

        Ok(())
    }

    fn flush(&self) {}
}
