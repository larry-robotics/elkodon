use crate::global_config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct StaticConfig {
    pub(crate) max_notifiers: usize,
    pub(crate) max_listeners: usize,
}

impl StaticConfig {
    pub fn new(config: &global_config::Entries) -> Self {
        Self {
            max_notifiers: config.defaults.event.max_notifiers,
            max_listeners: config.defaults.event.max_listeners,
        }
    }
}
