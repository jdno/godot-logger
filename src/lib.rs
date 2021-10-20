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
//! ```
//! use gdnative::prelude::*;
//! use godot_logger::GodotLogger;
//! use log::{Level, LevelFilter};
//!
//! fn init(handle: InitHandle) {
//!     GodotLogger::builder()
//!         .default_log_level(Level::Info)
//!         .add_filter("godot_logger", LevelFilter::Debug)
//!         .init();
//!     log::debug!("Initialized the logger");
//! }
//!
//! godot_init!(init);
//! ```
//!
//! The following will appear in the _Output_ console inside Godot:
//!
//! ```text
//! 2021-09-25 19:29:25 DEBUG godot_logger Initialized the logger
//! ```
//!
//! [env_logger]: https://crates.io/crates/env_logger
//! [gdnative]: https://crates.io/crates/gdnative
//! [godot-logger]: https://crates.io/crates/godot-logger
//! [`godot_print!`]: https://docs.rs/gdnative/latest/gdnative/macro.godot_print.html
//! [log]: https://crates.io/crates/log
//! [log4rs]: https://crates.io/crates/log4rs
//! [Godot]: https://godotengine.org/

pub use crate::builder::*;

mod appender;
mod builder;
mod filter;

/// A logger that prints to the output console of the Godot game engine
///
/// `GodotLogger` is a logger implementation that prints log records to the output console inside
/// the Godot game engine. The log level can be set per Rust module, similar to other logging
/// frameworks in Rust.
///
/// The recommended way to initialize the logger is by using the crate's [`Builder`]. Its setters
/// can be used to configure the logger and overwrite the default configuration.
///
/// # Examples
///
/// ```
/// use godot_logger::GodotLogger;
/// use log::{Level, LevelFilter};
///
/// // Configure and initialize the logger
/// GodotLogger::builder()
///     .default_log_level(Level::Debug)
///     .add_filter("godot-logger", LevelFilter::Warn)
///     .init();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct GodotLogger;

impl GodotLogger {
    pub fn builder() -> Builder {
        Builder::default()
    }
}
