use elkodon_cal::dynamic_storage::DynamicStorage;

use crate::service;
use crate::service::ServiceName;
use std::marker::PhantomData;

use super::listener::PortFactoryListener;
use super::notifier::PortFactoryNotifier;

#[derive(Debug)]
pub struct PortFactory<'config, Service: service::Details<'config>> {
    pub(crate) service: Service,
    _phantom_lifetime_b: PhantomData<&'config ()>,
}

unsafe impl<'config, Service: service::Details<'config>> Send for PortFactory<'config, Service> {}
unsafe impl<'config, Service: service::Details<'config>> Sync for PortFactory<'config, Service> {}

impl<'config, Service: service::Details<'config>> PortFactory<'config, Service> {
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

    pub fn number_of_listeners(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .event()
            .number_of_listeners()
    }

    pub fn number_of_notifiers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .event()
            .number_of_notifiers()
    }

    pub fn notifier<'a>(&'a self) -> PortFactoryNotifier<'a, 'config, Service> {
        PortFactoryNotifier::new(self)
    }

    pub fn listener<'a>(&'a self) -> PortFactoryListener<'a, 'config, Service> {
        PortFactoryListener { factory: self }
    }
}
