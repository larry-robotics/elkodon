use std::fmt::Debug;

use elkodon_bb_log::fail;

use crate::{
    port::subscriber::{Subscriber, SubscriberCreateError},
    service,
};

use super::publish_subscribe::PortFactory;

#[derive(Debug)]
pub struct PortFactorySubscriber<
    'factory,
    'config,
    Service: service::Details<'config>,
    MessageType: Debug,
> {
    pub(crate) factory: &'factory PortFactory<'config, Service, MessageType>,
}

impl<'factory, 'config, Service: service::Details<'config>, MessageType: Debug>
    PortFactorySubscriber<'factory, 'config, Service, MessageType>
{
    pub fn create(
        &self,
    ) -> Result<Subscriber<'factory, 'config, Service, MessageType>, SubscriberCreateError> {
        Ok(
            fail!(from self, when Subscriber::new(&self.factory.service, self.factory.service.state().static_config.publish_subscribe()),
                "Failed to create new Subscriber port."),
        )
    }
}
