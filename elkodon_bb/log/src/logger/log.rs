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
        let origin = format!("{}", origin);
        match log_level {
            LogLevel::Trace => log::trace!(target: &origin, "{}", formatted_message),
            LogLevel::Debug => log::debug!(target: &origin, "{}", formatted_message),
            LogLevel::Info => log::info!(target: &origin, "{}", formatted_message),
            LogLevel::Warn => log::warn!(target: &origin, "{}", formatted_message),
            LogLevel::Error => log::error!(target: &origin, "{}", formatted_message),
            LogLevel::Fatal => log::error!(target: &origin, "{}", formatted_message),
        }
    }
}
