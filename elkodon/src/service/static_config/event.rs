use crate::config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct StaticConfig {
    pub(crate) max_notifiers: usize,
    pub(crate) max_listeners: usize,
}

impl StaticConfig {
    pub(crate) fn new(config: &config::Config) -> Self {
        Self {
            max_notifiers: config.defaults.event.max_notifiers,
            max_listeners: config.defaults.event.max_listeners,
        }
    }
}
