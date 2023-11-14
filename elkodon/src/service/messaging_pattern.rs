use crate::service::static_config::event;
use crate::service::static_config::publish_subscribe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "messaging_pattern")]
pub enum MessagingPattern {
    PublishSubscribe(publish_subscribe::StaticConfig),
    Event(event::StaticConfig),
}

impl MessagingPattern {
    pub(crate) fn is_same_pattern(&self, rhs: &MessagingPattern) -> bool {
        match self {
            MessagingPattern::PublishSubscribe(_) => {
                matches!(rhs, MessagingPattern::PublishSubscribe(_))
            }
            MessagingPattern::Event(_) => {
                matches!(rhs, MessagingPattern::Event(_))
            }
        }
    }

    pub(crate) fn required_amount_of_samples_per_data_segment(
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
