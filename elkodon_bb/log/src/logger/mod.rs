//! Trait which can be implemented by logger, see [`crate::logger::console::Logger`]
//! for instance.

pub mod buffer;
pub mod console;

use std::fmt::Arguments;

use crate::LogLevel;

pub trait Logger: Send + Sync {
    /// logs a message
    fn log(&self, log_level: LogLevel, origin: Arguments, formatted_message: Arguments);
}
