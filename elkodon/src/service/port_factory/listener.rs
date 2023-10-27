use std::fmt::Debug;

use elkodon_bb_log::fail;

use crate::port::listener::{Listener, ListenerCreateError};
use crate::service;

use super::event::PortFactory;

#[derive(Debug)]
pub struct PortFactoryListener<'factory, 'global_config, Service: service::Details<'global_config>>
{
    pub(crate) factory: &'factory PortFactory<'global_config, Service>,
}

impl<'factory, 'global_config, Service: service::Details<'global_config>>
    PortFactoryListener<'factory, 'global_config, Service>
{
    pub fn create(
        &self,
    ) -> Result<Listener<'factory, 'global_config, Service>, ListenerCreateError> {
        Ok(fail!(from self, when Listener::new(&self.factory.service),
                    "Failed to create new Listener port."))
    }
}
