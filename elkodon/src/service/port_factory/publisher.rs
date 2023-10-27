use std::fmt::Debug;

use elkodon_bb_log::fail;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::{
    port::publisher::{Publisher, PublisherCreateError},
    service,
};

use super::publish_subscribe::PortFactory;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UnableToDeliverStrategy {
    Block,
    DiscardSample,
}

impl Serialize for UnableToDeliverStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&std::format!("{:?}", self))
    }
}

struct UnableToDeliverStrategyVisitor;

impl<'de> Visitor<'de> for UnableToDeliverStrategyVisitor {
    type Value = UnableToDeliverStrategy;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing either 'block' or 'discard_sample'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "block" => Ok(UnableToDeliverStrategy::Block),
            "discard_sample" => Ok(UnableToDeliverStrategy::DiscardSample),
            v => Err(E::custom(format!(
                "Invalid UnableToDeliverStrategy provided: \"{:?}\".",
                v
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for UnableToDeliverStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UnableToDeliverStrategyVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPublisherConfig {
    pub(crate) max_loaned_samples: usize,
    pub(crate) unable_to_deliver_strategy: UnableToDeliverStrategy,
}

#[derive(Debug)]
pub struct PortFactoryPublisher<
    'factory,
    'global_config,
    Service: service::Details<'global_config>,
    MessageType: Debug,
> {
    config: LocalPublisherConfig,
    pub(crate) factory: &'factory PortFactory<'global_config, Service, MessageType>,
}

impl<'factory, 'global_config, Service: service::Details<'global_config>, MessageType: Debug>
    PortFactoryPublisher<'factory, 'global_config, Service, MessageType>
{
    pub(crate) fn new(
        factory: &'factory PortFactory<'global_config, Service, MessageType>,
    ) -> Self {
        Self {
            config: LocalPublisherConfig {
                max_loaned_samples: factory
                    .service
                    .state()
                    .global_config
                    .defaults
                    .publish_subscribe
                    .publisher_max_loaned_samples,
                unable_to_deliver_strategy: factory
                    .service
                    .state()
                    .global_config
                    .defaults
                    .publish_subscribe
                    .unable_to_deliver_strategy,
            },
            factory,
        }
    }

    pub fn max_loaned_samples(mut self, value: usize) -> Self {
        self.config.max_loaned_samples = value;
        self
    }

    pub fn unable_to_deliver_strategy(mut self, value: UnableToDeliverStrategy) -> Self {
        self.config.unable_to_deliver_strategy = value;
        self
    }

    pub fn create(
        self,
    ) -> Result<Publisher<'factory, 'global_config, Service, MessageType>, PublisherCreateError>
    {
        Ok(
            fail!(from self, when Publisher::new(&self.factory.service, self.factory.service.state().static_config.publish_subscribe(), &self.config),
                "Failed to create new Publisher port."),
        )
    }
}
