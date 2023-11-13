pub mod event;
pub mod publish_subscribe;

use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_log::fatal_panic;
use elkodon_cal::hash::Hash;
use serde::{Deserialize, Serialize};

use crate::global_config;

use super::service_name::ServiceName;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "messaging_pattern")]
pub enum MessagingPattern {
    PublishSubscribe(publish_subscribe::StaticConfig),
    Event(event::StaticConfig),
}

impl MessagingPattern {
    pub fn is_same_pattern(&self, rhs: &MessagingPattern) -> bool {
        match self {
            MessagingPattern::PublishSubscribe(_) => {
                matches!(rhs, MessagingPattern::PublishSubscribe(_))
            }
            MessagingPattern::Event(_) => {
                matches!(rhs, MessagingPattern::Event(_))
            }
        }
    }

    pub fn required_amount_of_samples_per_data_segment(
        &self,
        publisher_max_loaned_samples: usize,
    ) -> usize {
        match self {
            MessagingPattern::PublishSubscribe(v) => {
                v.max_subscribers * (v.subscriber_buffer_size + v.subscriber_max_borrowed_samples)
                    + v.history_size
                    + publisher_max_loaned_samples
                    + 1
            }
            _ => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct StaticConfig {
    uuid: String,
    service_name: ServiceName,
    pub(crate) messaging_pattern: MessagingPattern,
}

impl StaticConfig {
    pub fn new_event<Hasher: Hash>(
        service_name: &ServiceName,
        config: &global_config::Config,
    ) -> Self {
        Self {
            uuid: Hasher::new(service_name.as_bytes()).as_hex_string(),
            service_name: *service_name,
            messaging_pattern: MessagingPattern::Event(event::StaticConfig::new(config)),
        }
    }

    pub fn new_publish_subscribe<Hasher: Hash>(
        service_name: &ServiceName,
        config: &global_config::Config,
    ) -> Self {
        Self {
            uuid: Hasher::new(service_name.as_bytes()).as_hex_string(),
            service_name: *service_name,
            messaging_pattern: MessagingPattern::PublishSubscribe(
                publish_subscribe::StaticConfig::new(config),
            ),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn service_name(&self) -> &ServiceName {
        &self.service_name
    }

    pub fn messaging_pattern(&self) -> &MessagingPattern {
        &self.messaging_pattern
    }

    pub(crate) fn has_same_messaging_pattern(&self, rhs: &StaticConfig) -> bool {
        self.messaging_pattern
            .is_same_pattern(&rhs.messaging_pattern)
    }

    pub(crate) fn event(&self) -> &event::StaticConfig {
        match &self.messaging_pattern {
            MessagingPattern::Event(ref v) => v,
            m => {
                fatal_panic!(from self, "This should never happen. Trying to access event::StaticConfig when the messaging pattern is actually {:?}!", m)
            }
        }
    }

    pub(crate) fn event_mut(&mut self) -> &mut event::StaticConfig {
        let origin = format!("{:?}", self);
        match &mut self.messaging_pattern {
            MessagingPattern::Event(ref mut v) => v,
            m => {
                fatal_panic!(from origin, "This should never happen. Trying to access event::StaticConfig when the messaging pattern is actually {:?}!", m)
            }
        }
    }

    pub(crate) fn publish_subscribe(&self) -> &publish_subscribe::StaticConfig {
        match &self.messaging_pattern {
            MessagingPattern::PublishSubscribe(ref v) => v,
            m => {
                fatal_panic!(from self, "This should never happen. Trying to access publish_subscribe::StaticConfig when the messaging pattern is actually {:?}!", m)
            }
        }
    }

    pub(crate) fn publish_subscribe_mut(&mut self) -> &mut publish_subscribe::StaticConfig {
        let origin = format!("{:?}", self);
        match &mut self.messaging_pattern {
            MessagingPattern::PublishSubscribe(ref mut v) => v,
            m => {
                fatal_panic!(from origin, "This should never happen. Trying to access publish_subscribe::StaticConfig when the messaging pattern is actually {:?}!", m)
            }
        }
    }
}
