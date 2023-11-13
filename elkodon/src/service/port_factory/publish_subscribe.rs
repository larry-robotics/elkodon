use std::{fmt::Debug, marker::PhantomData};

use elkodon_cal::dynamic_storage::DynamicStorage;

use crate::service;
use crate::service::service_name::ServiceName;

use super::{publisher::PortFactoryPublisher, subscriber::PortFactorySubscriber};

#[derive(Debug)]
pub struct PortFactory<'config, Service: service::Details<'config>, MessageType: Debug> {
    pub(crate) service: Service,
    _phantom_message_type: PhantomData<MessageType>,
    _phantom_lifetime_b: PhantomData<&'config ()>,
}

unsafe impl<'config, Service: service::Details<'config>, MessageType: Debug> Send
    for PortFactory<'config, Service, MessageType>
{
}
unsafe impl<'config, Service: service::Details<'config>, MessageType: Debug> Sync
    for PortFactory<'config, Service, MessageType>
{
}

impl<'config, Service: service::Details<'config>, MessageType: Debug>
    PortFactory<'config, Service, MessageType>
{
    pub(crate) fn new(service: Service) -> Self {
        Self {
            service,
            _phantom_message_type: PhantomData,
            _phantom_lifetime_b: PhantomData,
        }
    }

    pub fn name(&self) -> &ServiceName {
        self.service.state().static_config.service_name()
    }

    pub fn number_of_publishers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .number_of_publishers()
    }

    pub fn number_of_subscribers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .number_of_subscribers()
    }

    pub fn max_supported_publishers(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .max_publishers
    }

    pub fn max_supported_subscribers(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .max_subscribers
    }

    pub fn subscriber_buffer_size(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .subscriber_buffer_size
    }

    pub fn history_size(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .history_size
    }

    pub fn subscriber_max_borrowed_samples(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .subscriber_max_borrowed_samples
    }

    pub fn has_safe_overflow(&self) -> bool {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .enable_safe_overflow
    }

    pub fn subscriber<'a>(&'a self) -> PortFactorySubscriber<'a, 'config, Service, MessageType> {
        PortFactorySubscriber { factory: self }
    }

    pub fn publisher<'a>(&'a self) -> PortFactoryPublisher<'a, 'config, Service, MessageType> {
        PortFactoryPublisher::new(self)
    }
}
