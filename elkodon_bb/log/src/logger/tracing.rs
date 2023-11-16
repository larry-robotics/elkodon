use crate::LogLevel;

pub struct Logger {
    _priv: (),
}

impl Logger {
    pub const fn new() -> Self {
        Self { _priv: () }
    }
}

impl crate::logger::Logger for Logger {
    fn log(
        &self,
        log_level: crate::LogLevel,
        origin: std::fmt::Arguments,
        formatted_message: std::fmt::Arguments,
    ) {
        match log_level {
            LogLevel::Trace => tracing::trace!(origin, "{}", formatted_message),
            LogLevel::Debug => tracing::debug!(origin, "{}", formatted_message),
            LogLevel::Info => tracing::info!(origin, "{}", formatted_message),
            LogLevel::Warn => tracing::warn!(origin, "{}", formatted_message),
            LogLevel::Error => tracing::error!(origin, "{}", formatted_message),
            LogLevel::Fatal => tracing::error!(origin, "{}", formatted_message),
        }
    }
}
