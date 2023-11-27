//! # Example
//!
//! ## Simple Event Loop
//!
//! ```no_run
//! use core::time::Duration;
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! const CYCLE_TIME: Duration = Duration::from_secs(1);
//!
//! while let ElkEvent::Tick = Elk::wait(CYCLE_TIME) {
//!     // your algorithm in here
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Event Loop
//!
//! ```no_run
//! use core::time::Duration;
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! const CYCLE_TIME: Duration = Duration::from_secs(1);
//!
//! loop {
//!     match Elk::wait(CYCLE_TIME) {
//!         ElkEvent::Tick => {
//!             println!("entered next cycle");
//!         }
//!         ElkEvent::TerminationRequest => {
//!             println!("User pressed CTRL+c, terminating");
//!             break;
//!         }
//!         ElkEvent::InterruptSignal => {
//!             println!("Someone send an interrupt signal ...");
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use core::time::Duration;
use elkodon_bb_log::fatal_panic;
use elkodon_bb_posix::clock::{nanosleep, NanosleepError};
use elkodon_bb_posix::signal::SignalHandler;

/// A complete list of all events that can occur in the main event loop, [`Elk::wait()`].
pub enum ElkEvent {
    Tick,
    TerminationRequest,
    InterruptSignal,
}

/// The main event loop handling mechanism.
#[derive(Debug)]
#[non_exhaustive]
pub struct Elk {}

impl Elk {
    fn get_instance() -> &'static Self {
        static INSTANCE: Elk = Elk {};
        &INSTANCE
    }

    fn wait_impl(&self, cycle_time: Duration) -> ElkEvent {
        if SignalHandler::termination_requested() {
            return ElkEvent::TerminationRequest;
        }

        match nanosleep(cycle_time) {
            Ok(()) => {
                if SignalHandler::termination_requested() {
                    ElkEvent::TerminationRequest
                } else {
                    ElkEvent::Tick
                }
            }
            Err(NanosleepError::InterruptedBySignal(_)) => ElkEvent::InterruptSignal,
            Err(v) => {
                fatal_panic!(from self,
                    "Failed to wait with cycle time {:?} in main event look, caused by ({:?}).",
                    cycle_time, v);
            }
        }
    }

    /// Waits until an event has received. It returns
    /// [`ElkEvent::Tick`] when the `cycle_time` has passed, otherwise the other event that
    /// can occur.
    pub fn wait(cycle_time: Duration) -> ElkEvent {
        Self::get_instance().wait_impl(cycle_time)
    }
}
