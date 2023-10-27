use std::{fmt::Debug, ops::Deref, ptr::NonNull};

use crate::{message::Message, port::subscriber::Subscriber, service};

#[derive(Debug)]
pub struct Sample<
    'a,
    'subscriber,
    'global_config,
    Service: service::Details<'global_config>,
    Header: Debug,
    MessageType: Debug,
> {
    pub(crate) subscriber: &'subscriber Subscriber<'a, 'global_config, Service, MessageType>,
    pub(crate) ptr: NonNull<Message<Header, MessageType>>,
    pub(crate) channel_id: usize,
}

impl<
        'global_config,
        Service: service::Details<'global_config>,
        Header: Debug,
        MessageType: Debug,
    > Deref for Sample<'_, '_, 'global_config, Service, Header, MessageType>
{
    type Target = MessageType;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr.as_ptr()).data }
    }
}

impl<
        'a,
        'subscriber,
        'global_config,
        Service: service::Details<'global_config>,
        Header: Debug,
        MessageType: Debug,
    > Drop for Sample<'a, 'subscriber, 'global_config, Service, Header, MessageType>
{
    fn drop(&mut self) {
        self.subscriber
            .release_sample(self.channel_id, self.as_ptr());
    }
}

impl<
        'a,
        'subscriber,
        'global_config,
        Service: service::Details<'global_config>,
        Header: Debug,
        MessageType: Debug,
    > Sample<'a, 'subscriber, 'global_config, Service, Header, MessageType>
{
    pub fn as_ptr(&self) -> *const MessageType {
        &unsafe { self.ptr.as_ref() }.data
    }

    pub fn header(&self) -> &Header {
        &unsafe { self.ptr.as_ref() }.header
    }
}
