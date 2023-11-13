use std::fmt::Debug;

use crate::port::{
    event_id::EventId,
    notifier::{Notifier, NotifierCreateError},
};
use elkodon_bb_log::fail;

use crate::service;

use super::event::PortFactory;

#[derive(Debug)]
pub struct PortFactoryNotifier<'factory, 'config, Service: service::Details<'config>> {
    pub(crate) factory: &'factory PortFactory<'config, Service>,
    default_event_id: EventId,
}

impl<'factory, 'config, Service: service::Details<'config>>
    PortFactoryNotifier<'factory, 'config, Service>
{
    pub(crate) fn new(factory: &'factory PortFactory<'config, Service>) -> Self {
        Self {
            factory,
            default_event_id: EventId::default(),
        }
    }

    pub fn default_event_id(mut self, value: EventId) -> Self {
        self.default_event_id = value;
        self
    }

    pub fn create(&self) -> Result<Notifier<'factory, 'config, Service>, NotifierCreateError> {
        Ok(
            fail!(from self, when Notifier::new(&self.factory.service, self.default_event_id),
                    "Failed to create new Notifier port."),
        )
    }
}
