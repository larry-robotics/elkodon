//! # Example
//!
//! ```
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
//! let pubsub = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .open_or_create::<u64>()?;
//!
//! println!("name:                             {:?}", pubsub.name());
//! println!("number of active publishers:      {:?}", pubsub.number_of_publishers());
//! println!("number of active subscribers:     {:?}", pubsub.number_of_subscribers());
//! println!("max publishers:                   {:?}", pubsub.max_supported_publishers());
//! println!("max subscribers:                  {:?}", pubsub.max_supported_subscribers());
//! println!("subscriber buffer size:           {:?}", pubsub.subscriber_buffer_size());
//! println!("history size:                     {:?}", pubsub.history_size());
//! println!("subscriber max borrowed samples:  {:?}", pubsub.subscriber_max_borrowed_samples());
//! println!("safe overflow:                    {:?}", pubsub.has_safe_overflow());
//!
//! let publisher = pubsub.publisher().create()?;
//! let subscriber = pubsub.subscriber().create()?;
//!
//! # Ok(())
//! # }
//! ```

use std::{fmt::Debug, marker::PhantomData};

use elkodon_cal::dynamic_storage::DynamicStorage;

use crate::service;
use crate::service::service_name::ServiceName;

use super::{publisher::PortFactoryPublisher, subscriber::PortFactorySubscriber};

/// The factory for
/// [`MessagingPattern::PublishSubscribe`](crate::service::messaging_pattern::MessagingPattern::PublishSubscribe).
/// It can acquire dynamic and static service informations and create
/// [`crate::port::publisher::Publisher`]
/// or [`crate::port::subscriber::Subscriber`] ports.
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

    /// Returns the [`ServiceName`] of the service
    pub fn name(&self) -> &ServiceName {
        self.service.state().static_config.service_name()
    }

    /// Returns the number of active [`crate::port::publisher::Publisher`] ports.
    pub fn number_of_publishers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .number_of_publishers()
    }

    /// Returns the number of active [`crate::port::subscriber::Subscriber`] ports.
    pub fn number_of_subscribers(&self) -> usize {
        self.service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .number_of_subscribers()
    }

    /// Returns the maximum supported amount of [`crate::port::publisher::Publisher`] ports.
    pub fn max_supported_publishers(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .max_publishers
    }

    /// Returns the maximum supported amount of [`crate::port::subscriber::Subscriber`] ports.
    pub fn max_supported_subscribers(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .max_subscribers
    }

    /// Returns the maximum buffer size a [`crate::port::subscriber::Subscriber`] can have.
    pub fn subscriber_buffer_size(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .subscriber_buffer_size
    }

    /// Returns the maximum history size a [`crate::port::subscriber::Subscriber`] can request.
    pub fn history_size(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .history_size
    }

    /// Returns the maximum amount of samples a [`crate::port::subscriber::Subscriber`] can have.
    pub fn subscriber_max_borrowed_samples(&self) -> usize {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .subscriber_max_borrowed_samples
    }

    /// States if the services has safe overflow activated or not. Safe overflow means that the
    /// [`crate::port::publisher::Publisher`] will recycle the oldest samples when the buffer of
    /// the [`crate::port::subscriber::Subscriber`] is full.
    pub fn has_safe_overflow(&self) -> bool {
        self.service
            .state()
            .static_config
            .publish_subscribe()
            .enable_safe_overflow
    }

    /// Returns a [`PortFactorySubscriber`] to create a new
    /// [`crate::port::subscriber::Subscriber`] port.
    ///
    /// # Example
    ///
    /// ```
    /// use elkodon::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
    /// let pubsub = zero_copy::Service::new(&service_name)
    ///     .publish_subscribe()
    ///     .open_or_create::<u64>()?;
    ///
    /// let subscriber = pubsub.subscriber().create()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscriber<'a>(&'a self) -> PortFactorySubscriber<'a, 'config, Service, MessageType> {
        PortFactorySubscriber { factory: self }
    }

    /// Returns a [`PortFactoryPublisher`] to create a new
    /// [`crate::port::publisher::Publisher`] port.
    ///
    /// # Example
    ///
    /// ```
    /// use elkodon::prelude::*;
    /// use elkodon::service::port_factory::publisher::UnableToDeliverStrategy;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
    /// let pubsub = zero_copy::Service::new(&service_name)
    ///     .publish_subscribe()
    ///     .open_or_create::<u64>()?;
    ///
    /// let publisher = pubsub.publisher()
    ///                     .max_loaned_samples(6)
    ///                     .unable_to_deliver_strategy(UnableToDeliverStrategy::DiscardSample)
    ///                     .create()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn publisher<'a>(&'a self) -> PortFactoryPublisher<'a, 'config, Service, MessageType> {
        PortFactoryPublisher::new(self)
    }
}
