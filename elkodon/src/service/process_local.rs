use crate::service::dynamic_config::DynamicConfig;
use elkodon_cal::shm_allocator::pool_allocator::PoolAllocator;
use elkodon_cal::*;

use super::ServiceState;

#[derive(Debug)]
pub struct Service<'global_config> {
    state: ServiceState<
        'global_config,
        static_storage::process_local::Storage,
        dynamic_storage::process_local::Storage<DynamicConfig>,
    >,
}

impl<'global_config> crate::service::Service for Service<'global_config> {
    type Type<'b> = Service<'b>;
}

impl<'global_config> crate::service::Details<'global_config> for Service<'global_config> {
    type StaticStorage = static_storage::process_local::Storage;
    type ConfigSerializer = serialize::toml::Toml;
    type DynamicStorage = dynamic_storage::process_local::Storage<DynamicConfig>;
    type ServiceNameHasher = hash::sha1::Sha1;
    type SharedMemory = shared_memory::process_local::Memory<PoolAllocator>;
    type Connection = zero_copy_connection::process_local::Connection;
    type Event = event::process_local::Event<u64>;

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
