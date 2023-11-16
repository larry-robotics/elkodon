//! # Examples
//!
//! ```
//! use elkodon::prelude::*;
//! use elkodon::port::event_id::EventId;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_name = ServiceName::new(b"MyEventName")?;
//! let event = zero_copy::Service::new(&event_name)
//!     .event()
//!     .open_or_create()?;
//!
//! let listener = event.notifier()
//!                     .default_event_id(EventId::new(1234))
//!                     .create()?;
//! # Ok(())
//! # }
//! ```
use std::fmt::Debug;

use crate::port::{
    event_id::EventId,
    notifier::{Notifier, NotifierCreateError},
};
use elkodon_bb_log::fail;

use crate::service;

use super::event::PortFactory;

/// Factory to create a new [`Notifier`] port/endpoint for
/// [`MessagingPattern::Event`](crate::service::messaging_pattern::MessagingPattern::Event) based
/// communication.
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

    /// Sets a default [`EventId`] for the [`Notifier`] that is used in [`Notifier::notify()`]
    pub fn default_event_id(mut self, value: EventId) -> Self {
        self.default_event_id = value;
        self
    }

    /// Creates a new [`Notifier`] port or returns a [`NotifierCreateError`] on failure.
    pub fn create(&self) -> Result<Notifier<'factory, 'config, Service>, NotifierCreateError> {
        Ok(
            fail!(from self, when Notifier::new(&self.factory.service, self.default_event_id),
                    "Failed to create new Notifier port."),
        )
    }
}
