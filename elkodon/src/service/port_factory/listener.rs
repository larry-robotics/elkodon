use std::fmt::Debug;

use elkodon_bb_log::fail;

use crate::port::listener::{Listener, ListenerCreateError};
use crate::service;

use super::event::PortFactory;

#[derive(Debug)]
pub struct PortFactoryListener<'factory, 'config, Service: service::Details<'config>> {
    pub(crate) factory: &'factory PortFactory<'config, Service>,
}

impl<'factory, 'config, Service: service::Details<'config>>
    PortFactoryListener<'factory, 'config, Service>
{
    pub fn create(&self) -> Result<Listener<'factory, 'config, Service>, ListenerCreateError> {
        Ok(fail!(from self, when Listener::new(&self.factory.service),
                    "Failed to create new Listener port."))
    }
}
