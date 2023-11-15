//! # Examples
//!
//! ```
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_name = ServiceName::new(b"MyEventName")?;
//! let event = zero_copy::Service::new(&event_name)
//!     .event()
//!     .open_or_create()?;
//!
//! println!("name:                         {:?}", event.name());
//! println!("max listeners:                {:?}", event.max_supported_listeners());
//! println!("max notifiers:                {:?}", event.max_supported_notifiers());
//! println!("number of active listeners:   {:?}", event.number_of_listeners());
//! println!("number of active notifiers:   {:?}", event.number_of_notifiers());
//!
//! let listener = event.listener().create()?;
//! let notifier = event.notifier().create()?;
//! # Ok(())
//! # }
//! ```
use elkodon_cal::dynamic_storage::DynamicStorage;

use crate::service;
use crate::service::ServiceName;
use std::marker::PhantomData;

use super::listener::PortFactoryListener;
use super::notifier::PortFactoryNotifier;

/// The factory for
/// [`MessagingPattern::Event`](crate::service::messaging_pattern::MessagingPattern::Event). It can
/// acquire dynamic and static service informations and create [`crate::port::notifier::Notifier`]
/// or [`crate::port::listener::Listener`] ports.
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

    /// Returns the [`ServiceName`] of the [`crate::service::Service`]
    pub fn name(&self) -> &ServiceName {
        self.service.state().static_config.service_name()
    }

    /// Returns the maximum supported amount of [`crate::port::listener::Listener`]
    pub fn max_supported_listeners(&self) -> usize {
        self.service.state().static_config.event().max_listeners
    }

    /// Returns the maximum supported amount of [`crate::port::notifier::Notifier`]
    pub fn max_supported_notifiers(&self) -> usize {
        self.service.state().static_config.event().max_notifiers
    }

    /// Returns the number of active [`crate::port::listener::Listener`] ports
    pub fn number_of_listeners(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .event()
            .number_of_listeners()
    }

    /// Returns the number of active [`crate::port::notifier::Notifier`] ports
    pub fn number_of_notifiers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .event()
            .number_of_notifiers()
    }

    /// Returns a [`PortFactoryNotifier`] to create a new [`crate::port::notifier::Notifier`] port
    ///
    /// # Example
    ///
    /// ```
    /// use elkodon::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let event_name = ServiceName::new(b"MyEventName")?;
    /// let event = zero_copy::Service::new(&event_name)
    ///     .event()
    ///     .open_or_create()?;
    ///
    /// let notifier = event.notifier().create()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn notifier<'a>(&'a self) -> PortFactoryNotifier<'a, 'config, Service> {
        PortFactoryNotifier::new(self)
    }

    /// Returns a [`PortFactoryListener`] to create a new [`crate::port::listener::Listener`] port
    ///
    /// # Example
    ///
    /// ```
    /// use elkodon::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let event_name = ServiceName::new(b"MyEventName")?;
    /// let event = zero_copy::Service::new(&event_name)
    ///     .event()
    ///     .open_or_create()?;
    ///
    /// let listener = event.listener().create()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn listener<'a>(&'a self) -> PortFactoryListener<'a, 'config, Service> {
        PortFactoryListener { factory: self }
    }
}
