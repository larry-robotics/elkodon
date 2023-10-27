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
    'global_config,
    Service: service::Details<'global_config>,
    MessageType: Debug,
> {
    pub(crate) factory: &'factory PortFactory<'global_config, Service, MessageType>,
}

impl<'factory, 'global_config, Service: service::Details<'global_config>, MessageType: Debug>
    PortFactorySubscriber<'factory, 'global_config, Service, MessageType>
{
    pub fn create(
        &self,
    ) -> Result<Subscriber<'factory, 'global_config, Service, MessageType>, SubscriberCreateError>
    {
        Ok(
            fail!(from self, when Subscriber::new(&self.factory.service, self.factory.service.state().static_config.publish_subscribe()),
                "Failed to create new Subscriber port."),
        )
    }
}
