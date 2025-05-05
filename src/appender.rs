use chrono::Local;
#[cfg(feature = "godot3")]
use gdnative_core::{godot_print, godot_warn};
#[cfg(feature = "godot4")]
use godot::log::{godot_print, godot_warn};
use log::{Level, Record};
use log4rs::append::Append;

/// A log appender for `log4rs` that prints to Godot's output console
///
/// [godot-logger] uses [log4rs] under the hood to configure the logger. A custom appender has been
/// created to write log records to Godot's output console.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct GodotAppender;

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
