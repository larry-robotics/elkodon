//! # Elkodon
//!
//! TODO: link roadmap, examples, faq, github
//!
//! A service-oriented zero-copy lock-free inter-process communication middleware. It support
//! various messaging patterns like:
//!
//!  * Publish-Subscribe
//!  * Events
//!  * Request-Response (planned)
//!  * Pipeline (planned)
//!  * Blackboard (planned)
//!
//! Services identify uniquely by name and messaging pattern. They can be created with various
//! quality of service settings and shall be suitable in the future to be deployed in a `no_std`
//! and safety-critical environment.
//!
//! Furthermore, elkodon can be configured so that multiple service-setups can run on the same
//! machine or even in the same process without interferring with each other. This enables elkodon
//! also to run with other frameworks at the same time.
//!
//! Elkodon has its roots in the [eclipse iceoryx](https://github.com/eclipse-iceoryx/iceoryx)
//! project and overcame one of the major disadvantages, the central daemon. Elkodon has a complete
//! decentralized architecture and does not require a central daemon at all.
//!
//! # Examples
//!
//! Every service is identified uniquely by a [`crate::service::service_name::ServiceName`]. When
//! starting communicating one is required to create a service first that acts as a port factory.
//! With this factory, one can create then the endpoints of the service and can start to
//! communicate.
//!
//! ## Publish-Subscribe
//!
//! A simple publish-subscribe setup where the subscriber receives from the publisher until the
//! user terminates the processes by pressing `CTRL+c`.
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
//! **Listener**
//!
//! # Quality Of Services
//!  * required to calculate shm mem size
//!
//! # Custom Configuration
//!
//! # A Brief Tour Of Messaging Patterns

pub mod global_config;
pub(crate) mod message;
pub mod port;
pub mod sample;
pub mod sample_mut;
pub mod service;

#[doc(hidden)]
pub mod prelude {
    pub use crate::port::event_id::EventId;
    pub use crate::service::{
        process_local, service_name::ServiceName, zero_copy, Details, Service,
    };
    pub use elkodon_bb_container::semantic_string::SemanticString;
}
