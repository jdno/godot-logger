use log::LevelFilter;

/// A filter applies a custom log level to a Rust module
///
/// Logs in [godot-logger] can be filtered using the default log level or a module-level override.
/// Module-level overrides are configured using a `Filter`, which combines a module path in Rust
/// with a log level.
///
/// # Example
///
/// ```
/// use godot_logger::Filter;
/// use log::LevelFilter;
///
/// let filter = Filter::new("godot-logger", LevelFilter::Off);
/// ```
///
/// [godot-logger]: https://crates.io/crates/godot-logger
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Filter {
    module: &'static str,
    level: LevelFilter,
}

impl Filter {
    /// Initializes a new `Filter`
    ///
    /// Filters combine a module path in Rust with a log level, and are used to set a log level for
    /// a specific module.
    ///
    /// # Example
    ///
    /// ```
    /// use godot_logger::Filter;
    /// use log::LevelFilter;
    ///
    /// let filter = Filter::new("godot-logger", LevelFilter::Off);
    /// ```
    pub fn new(module: &'static str, level: LevelFilter) -> Self {
        Self { module, level }
    }

    /// Returns the filter's module path
    pub fn module(&self) -> &'static str {
        self.module
    }

    /// Returns the filter's log level
    pub fn level(&self) -> LevelFilter {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::Filter;
    use log::LevelFilter;

    #[test]
    fn module() {
        let filter = Filter::new("godot_logger::filter", LevelFilter::Debug);
        assert_eq!(filter.module(), "godot_logger::filter");
    }

    #[test]
    fn level() {
        let filter = Filter::new("godot_logger::filter", LevelFilter::Debug);
        assert!(matches!(filter.level(), LevelFilter::Debug));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Filter>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Filter>();
    }
}
