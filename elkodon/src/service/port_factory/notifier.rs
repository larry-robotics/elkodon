use std::fmt::Debug;

use crate::port::notifier::{Notifier, NotifierCreateError};
use elkodon_bb_log::fail;

use crate::service;

use super::event::PortFactory;

#[derive(Debug)]
pub struct PortFactoryNotifier<'factory, 'global_config, Service: service::Details<'global_config>>
{
    pub(crate) factory: &'factory PortFactory<'global_config, Service>,
    default_trigger_id: u64,
}

impl<'factory, 'global_config, Service: service::Details<'global_config>>
    PortFactoryNotifier<'factory, 'global_config, Service>
{
    pub(crate) fn new(factory: &'factory PortFactory<'global_config, Service>) -> Self {
        Self {
            factory,
            default_trigger_id: 0,
        }
    }

    pub fn default_trigger_id(mut self, value: u64) -> Self {
        self.default_trigger_id = value;
        self
    }

    pub fn create(
        &self,
    ) -> Result<Notifier<'factory, 'global_config, Service>, NotifierCreateError> {
        Ok(
            fail!(from self, when Notifier::new(&self.factory.service, self.default_trigger_id),
                    "Failed to create new Notifier port."),
        )
    }
}
