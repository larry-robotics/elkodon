use crate::{message::Message, port::publisher::Publisher, service};
use elkodon_cal::shared_memory::*;
use std::{fmt::Debug, mem::MaybeUninit, ptr::NonNull, sync::atomic::Ordering};

/// Acquired by a [`Publisher`] via [`Publisher::loan()`]. It stores the payload that will be sent
/// to all connected [`crate::port::subscriber::Subscriber`]s. If the [`SampleMut`] is not sent
/// it will release the loaned memory when going out of scope.
///
/// ## Notes
///
/// Does not implement [`Send`] since it releases unsent samples in the [`Publisher`] and the
/// [`Publisher`] is not thread-safe!
#[derive(Debug)]
pub struct SampleMut<
    'a,
    'publisher,
    'global_config,
    Service: service::Details<'global_config>,
    Header: Debug,
    MessageType: Debug,
> {
    publisher: &'publisher Publisher<'a, 'global_config, Service, MessageType>,
    ptr: NonNull<MaybeUninit<Message<Header, MessageType>>>,
    offset_to_chunk: PointerOffset,
}

impl<
        'global_config,
        Service: service::Details<'global_config>,
        Header: Debug,
        MessageType: Debug,
    > Drop for SampleMut<'_, '_, 'global_config, Service, Header, MessageType>
{
    fn drop(&mut self) {
        self.publisher.release_sample(self.offset_to_chunk);
        self.publisher.loan_counter.fetch_sub(1, Ordering::Relaxed);
    }
}

impl<
        'a,
        'publisher,
        'global_config,
        Service: service::Details<'global_config>,
        Header: Debug,
        MessageType: Debug,
    > SampleMut<'a, 'publisher, 'global_config, Service, Header, MessageType>
{
    pub(crate) fn new(
        publisher: &'publisher Publisher<'a, 'global_config, Service, MessageType>,
        ptr: NonNull<MaybeUninit<Message<Header, MessageType>>>,
        offset_to_chunk: PointerOffset,
    ) -> Self {
        publisher.loan_counter.fetch_add(1, Ordering::Relaxed);
        Self {
            publisher,
            ptr,
            offset_to_chunk,
        }
    }

    pub(crate) fn offset_to_chunk(&self) -> PointerOffset {
        self.offset_to_chunk
    }

    /// Returns a reference to the header of the sample. In publish subscribe communication the
    /// default header is [`crate::service::header::publish_subscribe::Header`].
    pub fn header(&self) -> &Header {
        &unsafe { &*self.ptr.as_ref().as_ptr() }.header
    }

    /// Returns a pointer to the underlying memory.
    pub fn as_ptr(&self) -> *const MessageType {
        &unsafe { &*self.ptr.as_ref().as_ptr() }.data
    }

    /// Returns a mutable pointer to the underlying memory.
    pub fn as_mut_ptr(&mut self) -> *mut MessageType {
        &mut unsafe { &mut *self.ptr.as_mut().as_mut_ptr() }.data
    }
}
