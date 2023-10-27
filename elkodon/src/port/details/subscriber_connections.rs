use std::cell::UnsafeCell;

use elkodon_bb_log::fail;
use elkodon_cal::named_concept::NamedConceptBuilder;
use elkodon_cal::zero_copy_connection::{
    ZeroCopyConnection, ZeroCopyConnectionBuilder, ZeroCopyCreationError, ZeroCopyPortDetails,
};

use crate::service::connection_config;
use crate::{
    global_config,
    port::port_identifiers::{UniquePublisherId, UniqueSubscriberId},
    service,
    service::{connection_name, static_config::publish_subscribe::StaticConfig},
};

#[derive(Debug)]
pub(crate) struct Connection<'global_config, Service: service::Details<'global_config>> {
    pub(crate) sender:
        <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::Sender,
}

impl<'global_config, Service: service::Details<'global_config>>
    Connection<'global_config, Service>
{
    fn new(
        this: &SubscriberConnections<'global_config, Service>,
        subscriber_id: UniqueSubscriberId,
    ) -> Result<Self, ZeroCopyCreationError> {
        let sender = fail!(from this, when <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::
                        Builder::new( &connection_name(this.port_id, subscriber_id))
                                .config(&connection_config::<Service>(this.global_config))
                                .buffer_size(this.static_config.subscriber_buffer_size)
                                .receiver_max_borrowed_samples(this.static_config.subscriber_max_borrowed_samples)
                                .enable_safe_overflow(this.static_config.enable_safe_overflow)
                                .create_sender(),
                        "Unable to establish connection to subscriber {:?} from publisher {:?}.",
                        subscriber_id, this.port_id);

        Ok(Self { sender })
    }
}

#[derive(Debug)]
pub(crate) struct SubscriberConnections<'global_config, Service: service::Details<'global_config>> {
    connections: Vec<UnsafeCell<Option<Connection<'global_config, Service>>>>,
    port_id: UniquePublisherId,
    global_config: &'global_config global_config::Entries,
    static_config: StaticConfig,
}

impl<'global_config, Service: service::Details<'global_config>>
    SubscriberConnections<'global_config, Service>
{
    pub(crate) fn new(
        capacity: usize,
        global_config: &'global_config global_config::Entries,
        port_id: UniquePublisherId,
        static_config: &StaticConfig,
    ) -> Self {
        Self {
            connections: (0..capacity).map(|_| UnsafeCell::new(None)).collect(),
            global_config,
            port_id,
            static_config: static_config.clone(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> &Option<Connection<'global_config, Service>> {
        unsafe { &(*self.connections[index].get()) }
    }

    // only used internally as convinience function
    #[allow(clippy::mut_from_ref)]
    fn get_mut(&self, index: usize) -> &mut Option<Connection<'global_config, Service>> {
        #[deny(clippy::mut_from_ref)]
        unsafe {
            &mut (*self.connections[index].get())
        }
    }

    pub(crate) fn remove(&self, index: usize) {
        *self.get_mut(index) = None
    }

    pub(crate) fn create(
        &self,
        index: usize,
        subscriber_id: UniqueSubscriberId,
    ) -> Result<bool, ZeroCopyCreationError> {
        if self.get(index).is_none() {
            *self.get_mut(index) = Some(Connection::new(self, subscriber_id)?);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn number_of_subscribers(&self) -> usize {
        self.connections
            .iter()
            .filter(|&connection| {
                let connection = unsafe { &*connection.get() };
                match connection {
                    None => false,
                    Some(c) => c.sender.is_connected(),
                }
            })
            .count()
    }

    pub(crate) fn len(&self) -> usize {
        self.connections.len()
    }

    pub(crate) fn capacity(&self) -> usize {
        self.connections.capacity()
    }
}
