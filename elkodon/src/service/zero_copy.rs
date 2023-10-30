use crate::port::event_id::EventId;
use crate::service::dynamic_config::DynamicConfig;
use elkodon_cal::shm_allocator::pool_allocator::PoolAllocator;
use elkodon_cal::*;

use super::ServiceState;

#[derive(Debug)]
pub struct Service<'global_config> {
    state: ServiceState<
        'global_config,
        static_storage::file::Storage,
        dynamic_storage::posix_shared_memory::Storage<DynamicConfig>,
    >,
}

impl<'global_config> crate::service::Service for Service<'global_config> {
    type Type<'b> = Service<'b>;
}

impl<'global_config> crate::service::Details<'global_config> for Service<'global_config> {
    type StaticStorage = static_storage::file::Storage;
    type ConfigSerializer = serialize::toml::Toml;
    type DynamicStorage = dynamic_storage::posix_shared_memory::Storage<DynamicConfig>;
    type ServiceNameHasher = hash::sha1::Sha1;
    type SharedMemory = shared_memory::posix::Memory<PoolAllocator>;
    type Connection = zero_copy_connection::posix_shared_memory::Connection;
    type Event = event::unix_datagram_socket::Event<EventId>;

    fn from_state(
        state: ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage>,
    ) -> Self {
        Self { state }
    }

    fn state(&self) -> &ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage> {
        &self.state
    }

    fn state_mut(
        &mut self,
    ) -> &mut ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage> {
        &mut self.state
    }
}
