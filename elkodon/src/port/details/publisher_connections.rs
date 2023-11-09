use std::cell::UnsafeCell;

use crate::{
    global_config,
    port::{
        port_identifiers::{UniquePublisherId, UniqueSubscriberId},
        publisher::{data_segment_config, data_segment_name},
    },
    service::{self, connection_config},
    service::{connection_name, static_config::publish_subscribe::StaticConfig},
};

use elkodon_bb_elementary::enum_gen;
use elkodon_bb_log::fail;
use elkodon_cal::named_concept::NamedConceptBuilder;
use elkodon_cal::{
    shared_memory::SharedMemory,
    shared_memory::{SharedMemoryBuilder, SharedMemoryOpenError},
    shm_allocator::pool_allocator::PoolAllocator,
    zero_copy_connection::*,
};

enum_gen! { ConnectionFailure
  mapping:
    ZeroCopyCreationError to FailedToEstablishConnection,
    SharedMemoryOpenError to UnableToMapPublishersDataSegment
}

impl std::fmt::Display for ConnectionFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}::{:?}", std::stringify!(Self), self)
    }
}

impl std::error::Error for ConnectionFailure {}

#[derive(Debug)]
pub(crate) struct Connection<'global_config, Service: service::Details<'global_config>> {
    pub(crate) receiver:
        <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::Receiver,
    pub(crate) data_segment: Service::SharedMemory,
}

impl<'global_config, Service: service::Details<'global_config>>
    Connection<'global_config, Service>
{
    fn new(
        this: &PublisherConnections<'global_config, Service>,
        publisher_id: UniquePublisherId,
    ) -> Result<Self, ConnectionFailure> {
        let msg = format!(
            "Unable to establish connection to publisher {:?} from subscriber {:?}.",
            publisher_id, this.subscriber_id
        );

        let receiver = fail!(from this,
                        when <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::
                            Builder::new( &connection_name(publisher_id, this.subscriber_id))
                                    .config(&connection_config::<Service>(this.global_config))
                                    .buffer_size(this.static_config.subscriber_buffer_size)
                                    .receiver_max_borrowed_samples(this.static_config.subscriber_max_borrowed_samples)
                                    .enable_safe_overflow(this.static_config.enable_safe_overflow)
                                    .create_receiver(),
                        "{} since the zero copy connection could not be established.", msg);

        let data_segment = fail!(from this,
                            when <Service::SharedMemory as SharedMemory<PoolAllocator>>::
                                Builder::new(&data_segment_name(publisher_id))
                                .config(&data_segment_config::<Service>(this.global_config))
                                .open(),
                            "{} since the publishers data segment could not be mapped into the process.", msg);

        Ok(Self {
            receiver,
            data_segment,
        })
    }
}
#[derive(Debug)]
pub(crate) struct PublisherConnections<'global_config, Service: service::Details<'global_config>> {
    connections: Vec<UnsafeCell<Option<Connection<'global_config, Service>>>>,
    subscriber_id: UniqueSubscriberId,
    global_config: &'global_config global_config::Entries,
    static_config: StaticConfig,
}

impl<'global_config, Service: service::Details<'global_config>>
    PublisherConnections<'global_config, Service>
{
    pub(crate) fn new(
        capacity: usize,
        subscriber_id: UniqueSubscriberId,
        global_config: &'global_config global_config::Entries,
        static_config: &StaticConfig,
    ) -> Self {
        Self {
            connections: (0..capacity).map(|_| UnsafeCell::new(None)).collect(),
            subscriber_id,
            global_config,
            static_config: static_config.clone(),
        }
    }

    pub(crate) fn subscriber_id(&self) -> UniqueSubscriberId {
        self.subscriber_id
    }

    pub(crate) fn get(&self, index: usize) -> &Option<Connection<'global_config, Service>> {
        unsafe { &*self.connections[index].get() }
    }

    // only used internally as convinience function
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn get_mut(&self, index: usize) -> &mut Option<Connection<'global_config, Service>> {
        #[deny(clippy::mut_from_ref)]
        unsafe {
            &mut *self.connections[index].get()
        }
    }

    pub(crate) fn create(
        &self,
        index: usize,
        publisher_id: UniquePublisherId,
    ) -> Result<(), ConnectionFailure> {
        if self.get(index).is_none() {
            *self.get_mut(index) = Some(Connection::new(self, publisher_id)?);
        }

        Ok(())
    }

    pub(crate) fn remove(&self, index: usize) {
        *self.get_mut(index) = None;
    }

    pub(crate) fn len(&self) -> usize {
        self.connections.len()
    }

    pub(crate) fn capacity(&self) -> usize {
        self.connections.capacity()
    }
}
