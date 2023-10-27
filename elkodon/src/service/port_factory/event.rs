use crate::service;
use crate::service::ServiceName;
use std::marker::PhantomData;

use super::listener::PortFactoryListener;
use super::notifier::PortFactoryNotifier;

#[derive(Debug)]
pub struct PortFactory<'global_config, Service: service::Details<'global_config>> {
    pub(crate) service: Service,
    _phantom_lifetime_b: PhantomData<&'global_config ()>,
}

unsafe impl<'global_config, Service: service::Details<'global_config>> Send
    for PortFactory<'global_config, Service>
{
}
unsafe impl<'global_config, Service: service::Details<'global_config>> Sync
    for PortFactory<'global_config, Service>
{
}

impl<'global_config, Service: service::Details<'global_config>>
    PortFactory<'global_config, Service>
{
    pub(crate) fn new(service: Service) -> Self {
        Self {
            service,
            _phantom_lifetime_b: PhantomData,
        }
    }

    pub fn name(&self) -> &ServiceName {
        self.service.state().static_config.service_name()
    }

    pub fn max_supported_listeners(&self) -> usize {
        self.service.state().static_config.event().max_listeners
    }

    pub fn max_supported_notifiers(&self) -> usize {
        self.service.state().static_config.event().max_notifiers
    }

    pub fn notifier<'a>(&'a self) -> PortFactoryNotifier<'a, 'global_config, Service> {
        PortFactoryNotifier::new(self)
    }

    pub fn listener<'a>(&'a self) -> PortFactoryListener<'a, 'global_config, Service> {
        PortFactoryListener { factory: self }
    }
}
