use std::{fmt::Debug, ops::Deref, ptr::NonNull};

use crate::{message::Message, port::subscriber::Subscriber, service};

/// It stores the payload and is acquired by the [`Subscriber`] whenever it receives new data from a
/// [`crate::port::publisher::Publisher`] via [`Subscriber::receive()`].
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
        unsafe { &self.ptr.as_ref().data }
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
            .release_sample(self.channel_id, self.payload());
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
    /// Returns a reference to the payload of the sample
    pub fn payload(&self) -> &MessageType {
        &unsafe { self.ptr.as_ref() }.data
    }

    /// Returns a reference to the header of the sample. In publish subscribe communication the
    /// default header is [`crate::service::header::publish_subscribe::Header`].
    pub fn header(&self) -> &Header {
        &unsafe { self.ptr.as_ref() }.header
    }
}
