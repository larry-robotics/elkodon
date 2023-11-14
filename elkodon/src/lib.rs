//! # Elkodon
//!
//! Elkodon is a cutting-edge service-oriented zero-copy lock-free inter-process communication
//! middleware. Designed to support various
//! [`MessagingPattern`](crate::service::messaging_pattern::MessagingPattern)s
//! Elkodon empowers developers with
//! the flexibility of:
//!
//! - Publish-Subscribe
//! - Events
//! - Request-Response (planned)
//! - Pipeline (planned)
//! - Blackboard (planned)
//!
//! For a comprehensive list of all planned features, please refer to the
//! [GitHub Roadmap](https://github.com/elkodon/elkodon/ROADMAP.md).
//!
//! Services are uniquely identified by name and
//! [`MessagingPattern`](crate::service::messaging_pattern::MessagingPattern). They can be instantiated with
//! diverse quality-of-service settings and are envisioned to be deployable in a `no_std` and
//! safety-critical environment in the future.
//!
//! Moreover, Elkodon offers configuration options that enable multiple service setups to coexist
//! on the same machine or even within the same process without interference. This versatility
//! allows Elkodon to seamlessly integrate with other frameworks simultaneously.
//!
//! Elkodon traces its lineage back to the
//! [eclipse iceoryx](https://github.com/eclipse-iceoryx/iceoryx) project, addressing a major
//! drawback – the central daemon. Elkodon embraces a fully decentralized architecture,
//! eliminating the need for a central daemon entirely.
//!
//! # Examples
//!
//! Each service is uniquely identified by a [`ServiceName`](crate::service::service_name::ServiceName).
//! Initiating communication requires the creation of a service, which serves as a port factory.
//! With this factory, endpoints for the service can be created, enabling seamless communication.
//!
//! For more detailed examples, explore the
//! [GitHub example folder](https://github.com/elkodon/elkodon/tree/main/examples).
//!
//! ## Publish-Subscribe
//!
//! Explore a simple publish-subscribe setup where the subscriber continuously receives data from
//! the publisher until the processes are gracefully terminated by the user with `CTRL+C`.
//!
//! **Subscriber (Process 1)**
//!
//! ```no_run
//! use elkodon::prelude::*;
//! use elkodon_bb_posix::signal::SignalHandler;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
//!
//! // create our port factory by creating or opening the service
//! let service = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .open_or_create::<u64>()?;
//!
//! let subscriber = service.subscriber().create()?;
//!
//! while !SignalHandler::termination_requested() {
//!     while let Some(sample) = subscriber.receive()? {
//!         println!("received: {:?}", *sample);
//!     }
//!
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }
//! # Ok(())
//! # }
//! ```
//!
//! **Publisher (Process 2)**
//!
//! ```no_run
//! use elkodon::prelude::*;
//! use elkodon_bb_posix::signal::SignalHandler;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"My/Funk/ServiceName").unwrap();
//!
//! // create our port factory by creating or opening the service
//! let service = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .open_or_create::<u64>()?;
//!
//! let publisher = service.publisher().create()?;
//!
//! while !SignalHandler::termination_requested() {
//!     let mut sample = publisher.loan()?;
//!     unsafe { sample.as_mut_ptr().write(1234) };
//!     publisher.send(sample)?;
//!
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Events
//!
//! Explore a straightforward event setup, where the listener patiently awaits events from the
//! notifier. This continuous event listening continues until the user gracefully terminates
//! the processes by pressing `CTRL+C`.
//!
//! **Listener (Process 1)**
//!
//! ```no_run
//! use elkodon::prelude::*;
//! use elkodon_bb_posix::signal::SignalHandler;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_name = ServiceName::new(b"MyEventName")?;
//!
//! let event = zero_copy::Service::new(&event_name)
//!     .event()
//!     .open_or_create()?;
//!
//! let mut listener = event.listener().create()?;
//!
//! while !SignalHandler::termination_requested() {
//!     for event_id in listener.timed_wait(std::time::Duration::from_secs(1))? {
//!         println!("event was triggered with id: {:?}", event_id);
//!     }
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! **Notifier (Process 2)**
//!
//! ```no_run
//! use elkodon::prelude::*;
//! use elkodon_bb_posix::signal::SignalHandler;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_name = ServiceName::new(b"MyEventName")?;
//!
//! let event = zero_copy::Service::new(&event_name)
//!     .event()
//!     .open_or_create()?;
//!
//! let notifier = event.notifier().create()?;
//!
//! let mut counter: u64 = 0;
//! while !SignalHandler::termination_requested() {
//!     counter += 1;
//!     notifier.notify_with_custom_event_id(EventId::new(counter))?;
//!
//!     println!("Trigger event with id {} ...", counter);
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Quality Of Services
//!
//! Quality of service settings, or service settings, play a crucial role in determining memory
//! allocation in a worst-case scenario. These settings can be configured during the creation of
//! a service, immediately after defining the
//! [`MessagingPattern`](crate::service::messaging_pattern::MessagingPattern). In cases where the service
//! already exists, these settings are interpreted as minimum requirements, ensuring a flexible
//! and dynamic approach to memory management.
//!
//! ## Publish-Subscribe
//!
//! ```
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"PubSubQos")?;
//!
//! let service = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .enable_safe_overflow(true)
//!     // how many samples a subscriber can borrow in parallel
//!     .subscriber_max_borrowed_samples(2)
//!     // the maximum history size a subscriber can request
//!     .history_size(3)
//!     // the maximum buffer size of a subscriber
//!     .subscriber_buffer_size(4)
//!     // the maximum amount of subscribers of this service
//!     .max_subscribers(5)
//!     // the maximum amount of publishers of this service
//!     .max_publishers(2)
//!     .create::<u64>()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Event
//!
//! ```
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_name = ServiceName::new(b"EventQos")?;
//!
//! let event = zero_copy::Service::new(&event_name)
//!     .event()
//!     // the maximum amount of notifiers of this service
//!     .max_notifiers(2)
//!     // the maximum amount of listeners of this service
//!     .max_listeners(2)
//!     .create()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Port Behavior
//!
//! Certain ports in Elkodon provide users with the flexibility to define custom behaviors in
//! specific situations.
//! Custom port behaviors can be specified during the creation of a port,
//! utilizing the port factory or service, immediately following the specification of the port
//! type. This feature enhances the adaptability of Elkodon to diverse use cases and scenarios.
//!
//! ```
//! use elkodon::prelude::*;
//! use elkodon::service::port_factory::publisher::UnableToDeliverStrategy;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
//!
//! let service = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .enable_safe_overflow(false)
//!     .open_or_create::<u64>()?;
//!
//! let publisher = service.publisher()
//!     // the maximum amount of samples this publisher can loan in parallel
//!     .max_loaned_samples(2)
//!     // defines the behavior when a sample could not be delivered when the subscriber buffer is
//!     // full, only useful in an non-overflow scenario
//!     .unable_to_deliver_strategy(UnableToDeliverStrategy::DiscardSample)
//!     .create()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Custom Configuration
//!
//! Elkodon offers the flexibility to configure default quality of service settings, paths, and
//! file suffixes through a custom configuration file.
//!
//! For in-depth details and examples, please visit the
//! [GitHub config folder](https://github.com/elkodon/elkodon/tree/main/config).

/// Handles Elkodons global configuration
pub mod config;

pub(crate) mod message;

/// The ports or communication endpoints of Elkodon
pub mod port;

/// The payload that is received by a [`crate::port::subscriber::Subscriber`].
pub mod sample;

/// The payload that is sent by a [`crate::port::publisher::Publisher`].
pub mod sample_mut;

/// The foundation of communication the service with its
/// [`MessagingPattern`](crate::service::messaging_pattern::MessagingPattern)
pub mod service;

#[doc(hidden)]
pub mod prelude {
    pub use crate::port::event_id::EventId;
    pub use crate::service::{
        process_local, service_name::ServiceName, zero_copy, Details, Service,
    };
    pub use elkodon_bb_container::semantic_string::SemanticString;
}
