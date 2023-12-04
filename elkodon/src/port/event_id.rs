//! # Example
//!
//! ```
//! use elkodon::prelude::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let event_name = ServiceName::new("MyEventName")?;
//! # let event = zero_copy::Service::new(&event_name)
//! #     .event()
//! #     .open_or_create()?;
//!
//! let mut listener = event.listener().create()?;
//! let mut notifier = event.notifier()
//!     .default_event_id(EventId::new(123))
//!     .create()?;
//!
//! // notify the listener with default event id 123
//! notifier.notify()?;
//!
//! notifier.notify_with_custom_event_id(EventId::new(456));
//!
//! for event_id in listener.try_wait()? {
//!     println!("event was triggered with id: {:?}", event_id);
//! }
//!
//! # Ok(())
//! # }
//! ```

use elkodon_cal::event::TriggerId;

/// Id to identify the source in event based communication.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EventId(u64);

impl EventId {
    /// Creates a new [`EventId`] from a given integer value.
    pub fn new(value: u64) -> Self {
        EventId(value)
    }

    /// Returns the underlying integer value of the [`EventId`].
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl TriggerId for EventId {}
