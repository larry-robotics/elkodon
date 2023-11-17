//! # Example
//!
//! ```
//! use elkodon::prelude::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let service_name = ServiceName::new(b"My/Funk/ServiceName").unwrap();
//! #
//! # let service = zero_copy::Service::new(&service_name)
//! #     .publish_subscribe()
//! #     .open_or_create::<u64>()?;
//! #
//! # let publisher = service.publisher().create()?;
//!
//! let mut sample = publisher.loan()?;
//! sample.payload_mut().write(1234);
//! let sample = unsafe { sample.assume_init() };
//!
//! println!("timestamp: {:?}, publisher port id: {:?}",
//!     sample.header().time_stamp(), sample.header().publisher_id());
//! publisher.send(sample)?;
//!
//! # Ok(())
//! # }
//! ```

use crate::{port::publisher::Publisher, raw_sample::RawSampleMut, service};
use elkodon_cal::shared_memory::*;
use std::{fmt::Debug, mem::MaybeUninit, sync::atomic::Ordering};

/// Acquired by a [`Publisher`] via [`Publisher::loan()`]. It stores the payload that will be sent
/// to all connected [`crate::port::subscriber::Subscriber`]s. If the [`SampleMut`] is not sent
/// it will release the loaned memory when going out of scope.
///
/// # Notes
///
/// Does not implement [`Send`] since it releases unsent samples in the [`Publisher`] and the
/// [`Publisher`] is not thread-safe!
#[derive(Debug)]
pub struct SampleMut<
    'a,
    'publisher,
    'config,
    Service: service::Details<'config>,
    Header: Debug,
    MessageType: Debug,
> {
    publisher: &'publisher Publisher<'a, 'config, Service, MessageType>,
    ptr: RawSampleMut<Header, MessageType>,
    offset_to_chunk: PointerOffset,
}

impl<'config, Service: service::Details<'config>, Header: Debug, MessageType: Debug> Drop
    for SampleMut<'_, '_, 'config, Service, Header, MessageType>
{
    fn drop(&mut self) {
        self.publisher.release_sample(self.offset_to_chunk);
        self.publisher.loan_counter.fetch_sub(1, Ordering::Relaxed);
    }
}

impl<
        'a,
        'publisher,
        'config,
        Service: service::Details<'config>,
        Header: Debug,
        MessageType: Debug,
    > SampleMut<'a, 'publisher, 'config, Service, Header, MaybeUninit<MessageType>>
{
    pub(crate) fn new(
        publisher: &'publisher Publisher<'a, 'config, Service, MessageType>,
        ptr: RawSampleMut<Header, MaybeUninit<MessageType>>,
        offset_to_chunk: PointerOffset,
    ) -> Self {
        publisher.loan_counter.fetch_add(1, Ordering::Relaxed);

        // SAFETY: the transmute is not nice but safe since MaybeUninit is #[repr(transparent)} to the inner type
        let publisher = unsafe { std::mem::transmute(publisher) };

        Self {
            publisher,
            ptr,
            offset_to_chunk,
        }
    }

    /// Extracts the value of the `MaybeUninit<MessageType>` container and labels the sample as initialized
    ///
    /// # Safety
    ///
    /// The caller must ensure that `MaybeUninit<MessageType>` really is initialized. Calling this when
    /// the content is not fully initialized causes immediate undefined behavior.
    pub unsafe fn assume_init(
        self,
    ) -> SampleMut<'a, 'publisher, 'config, Service, Header, MessageType> {
        // the transmute is not nice but safe since MaybeUninit is #[repr(transparent)] to the inner type
        std::mem::transmute(self)
    }
}

impl<
        'a,
        'publisher,
        'config,
        Service: service::Details<'config>,
        Header: Debug,
        M: Debug, // M is either MaybeUninit<MessageType> or MessageType
    > SampleMut<'a, 'publisher, 'config, Service, Header, M>
{
    pub(crate) fn offset_to_chunk(&self) -> PointerOffset {
        self.offset_to_chunk
    }

    /// Returns a reference to the header of the sample. In publish subscribe communication the
    /// default header is [`crate::service::header::publish_subscribe::Header`].
    pub fn header(&self) -> &Header {
        self.ptr.as_header_ref()
    }

    /// Returns a reference to the payload of the sample.
    pub fn payload(&self) -> &M {
        self.ptr.as_data_ref()
    }

    /// Returns a mutable reference to the payload of the sample.
    pub fn payload_mut(&mut self) -> &mut M {
        self.ptr.as_data_mut()
    }
}
