use elkodon_cal::shared_memory::*;
use std::{fmt::Debug, mem::MaybeUninit, ptr::NonNull, sync::atomic::Ordering};

use crate::{message::Message, port::publisher::Publisher, service};

/// # Important
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

    pub fn header(&self) -> &Header {
        &unsafe { &*self.ptr.as_ref().as_ptr() }.header
    }

    pub fn as_ptr(&self) -> *const MessageType {
        &unsafe { &*self.ptr.as_ref().as_ptr() }.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut MessageType {
        &mut unsafe { &mut *self.ptr.as_mut().as_mut_ptr() }.data
    }
}
