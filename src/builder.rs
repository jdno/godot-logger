use log::{Level, LevelFilter, SetLoggerError};
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;

use crate::appender::GodotAppender;
use crate::filter::Filter;

const APPENDER_NAME: &str = "godot-logger";

/// A `Builder` that configures and initializes the Godot logger
///
/// [godot-logger] implements the builder pattern as the primary interface to configure and
/// initialize the logger. The configuration has sensible defaults that can be overwritten by
/// calling the corresponding setters on the `Builder` struct. Once the configuration is done, the
/// logger can be initialized by calling the `build` method.
///
/// # Examples
///
/// ```
/// use log::Level;
/// use godot_logger::GodotLogger;
///
/// GodotLogger::builder()
///     .default_log_level(Level::Debug)
///     .init();
/// ```
///
/// [godot-logger]: https://crates.io/crates/godot-logger
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Builder {
    default_log_level: Level,
    filters: Vec<Filter>,
}

impl Builder {
    /// Sets the default log level
    ///
    /// `GodotLogger` matches all log records against a default log level. By default, only warnings
    /// and errors are logged.
    ///
    /// # Examples
    ///
    /// ```
    /// use log::Level;
    /// use godot_logger::GodotLogger;
    ///
    /// let mut builder = GodotLogger::builder();
    /// builder = builder.default_log_level(Level::Debug);
    /// ```
    pub fn default_log_level(mut self, default_log_level: Level) -> Self {
        self.default_log_level = default_log_level;
        self
    }

    /// Adds a filter
    ///
    /// Filters override the default log level for specific Rust modules.
    ///
    /// # Examples
    ///
    /// ```
    /// use godot_logger::GodotLogger;
    /// use log::LevelFilter;
    ///
    /// GodotLogger::builder().add_filter("godot_logger", LevelFilter::Off);
    /// ```
    pub fn add_filter(mut self, module: &'static str, level: LevelFilter) -> Self {
        self.filters.push(Filter::new(module, level));
        self
    }

    /// Initializes the logger
    ///
    /// This method consumes the builder and initializes the logger with the current configuration
    /// of the builder. After calling this method, log records will be written to Godot's output
    /// console.
    ///
    /// # Examples
    ///
    /// ```
    /// use log::Level;
    /// use godot_logger::GodotLogger;
    ///
    /// GodotLogger::builder().init();
    /// ```
    pub fn init(self) -> Result<(), SetLoggerError> {
        let loggers: Vec<Logger> = self
            .filters
            .iter()
            .map(|filter| {
                Logger::builder()
                    .appender(APPENDER_NAME)
                    .build(filter.module(), filter.level())
            })
            .collect();

        let config = Config::builder()
            .appender(Appender::builder().build(APPENDER_NAME, Box::new(GodotAppender)))
            .loggers(loggers)
            .build(
                Root::builder()
                    .appender(APPENDER_NAME)
                    .build(self.default_log_level.to_level_filter()),
            )
            .unwrap();

        let _handle = log4rs::init_config(config)?;
        Ok(())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            default_log_level: Level::Warn,
            filters: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use log::{Level, LevelFilter};

    use super::Builder;

    #[test]
    fn default_log_level() {
        let mut builder = Builder::default();

        builder = builder.default_log_level(Level::Debug);

        assert!(matches!(builder.default_log_level, Level::Debug));
    }

    #[test]
    fn add_filter() {
        let mut builder = Builder::default();

        builder = builder.add_filter("godot_logger::builder", LevelFilter::Off);

        assert_eq!(builder.filters.len(), 1);
    }

    #[test]
    fn trait_default() {
        let builder = Builder::default();
        assert!(matches!(builder.default_log_level, Level::Warn));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Builder>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Builder>();
    }
}
