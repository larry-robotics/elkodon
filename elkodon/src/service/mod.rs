pub mod builder;
pub mod dynamic_config;
pub mod header;
pub mod port_factory;
pub mod static_config;

pub mod process_local;
pub mod service_name;
pub mod zero_copy;

use std::fmt::Debug;

use crate::global_config;
use crate::port::event_id::EventId;
use crate::port::port_identifiers::{UniqueListenerId, UniquePublisherId, UniqueSubscriberId};
use crate::service::dynamic_config::DynamicConfig;
use crate::service::static_config::*;
use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_log::{fail, fatal_panic, trace, warn};
use elkodon_bb_system_types::file_name::FileName;
use elkodon_bb_system_types::path::Path;
use elkodon_cal::dynamic_storage::DynamicStorage;
use elkodon_cal::event::Event;
use elkodon_cal::hash::Hash;
use elkodon_cal::named_concept::NamedConceptListError;
use elkodon_cal::named_concept::*;
use elkodon_cal::serialize::Serialize;
use elkodon_cal::shared_memory::SharedMemory;
use elkodon_cal::shm_allocator::pool_allocator::PoolAllocator;
use elkodon_cal::static_storage::*;
use elkodon_cal::zero_copy_connection::ZeroCopyConnection;

use self::builder::Builder;
use self::dynamic_config::DecrementReferenceCounterResult;
use self::service_name::ServiceName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceDoesExistError {
    InsufficientPermissions,
    InternalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceListError {
    InsufficientPermissions,
    InternalError,
}

pub(crate) fn event_concept_name(listener_id: &UniqueListenerId) -> FileName {
    let msg = "The system does not support the required file name length for the listeners event concept name.";
    let origin = "event_concept_name()";
    let mut file = fatal_panic!(from origin, when FileName::new(listener_id.0.pid().to_string().as_bytes()), "{}", msg);
    fatal_panic!(from origin, when file.push(b'_'), "{}", msg);
    fatal_panic!(from origin, when file.push_bytes(listener_id.0.value().to_string().as_bytes()), "{}", msg);
    file
}

pub(crate) fn dynamic_config_storage_name(static_config: &StaticConfig) -> FileName {
    FileName::new(static_config.uuid().as_bytes()).unwrap()
}

pub(crate) fn dynamic_config_storage_config<
    'global_config,
    Service: crate::service::Details<'global_config>,
>(
    global_config: &global_config::Entries,
) -> <Service::DynamicStorage as NamedConceptMgmt>::Configuration {
    let origin = "dynamic_config_storage_config()";

    let f = match FileName::new(
        global_config
            .global
            .service
            .dynamic_config_storage_suffix
            .as_bytes(),
    ) {
        Err(_) => {
            fatal_panic!(from origin, "The dynamic_config_storage_suffix \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.service.dynamic_config_storage_suffix);
        }
        Ok(v) => v,
    };

    <Service::DynamicStorage as NamedConceptMgmt>::Configuration::default().suffix(f)
}

pub(crate) fn static_config_storage_name(uuid: &str) -> FileName {
    FileName::new(uuid.as_bytes()).unwrap()
}

pub(crate) fn static_config_storage_config<
    'global_config,
    Service: crate::service::Details<'global_config>,
>(
    global_config: &global_config::Entries,
) -> <Service::StaticStorage as NamedConceptMgmt>::Configuration {
    let origin = "dynamic_config_storage_config()";

    let f = match FileName::new(
        global_config
            .global
            .service
            .static_config_storage_suffix
            .as_bytes(),
    ) {
        Err(_) => {
            fatal_panic!(from origin, "The static_config_storage_suffix \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.service.static_config_storage_suffix);
        }
        Ok(v) => v,
    };

    let mut path_hint = match Path::new(global_config.global.root_path.as_bytes()) {
        Err(_) => {
            fatal_panic!(from origin, "The root_path \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.root_path);
        }
        Ok(v) => v,
    };

    if path_hint
        .push_bytes(global_config.global.service.directory.as_bytes())
        .is_err()
    {
        fatal_panic!(from origin, "The service.directory \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.service.directory);
    }

    <Service::StaticStorage as NamedConceptMgmt>::Configuration::default()
        .suffix(f)
        .path_hint(path_hint)
}

pub(crate) fn connection_name(
    publisher_id: UniquePublisherId,
    subscriber_id: UniqueSubscriberId,
) -> FileName {
    let mut file = FileName::new(publisher_id.0.value().to_string().as_bytes()).unwrap();
    file.push(b'_').unwrap();
    file.push_bytes(subscriber_id.0.value().to_string().as_bytes())
        .unwrap();
    file
}

pub(crate) fn connection_config<
    'global_config,
    Service: crate::service::Details<'global_config>,
>(
    global_config: &global_config::Entries,
) -> <Service::Connection as NamedConceptMgmt>::Configuration {
    let origin = "connection_config()";

    let f = match FileName::new(global_config.global.service.connection_suffix.as_bytes()) {
        Err(_) => {
            fatal_panic!(from origin, "The connection_suffix \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.service.connection_suffix);
        }
        Ok(v) => v,
    };

    <Service::Connection as NamedConceptMgmt>::Configuration::default().suffix(f)
}
#[derive(Debug)]
pub struct ServiceState<
    'global_config,
    Static: StaticStorage,
    Dynamic: DynamicStorage<DynamicConfig>,
> {
    pub(crate) static_config: StaticConfig,
    pub(crate) global_config: &'global_config global_config::Entries,
    pub(crate) dynamic_storage: Dynamic,
    pub(crate) static_storage: Static,
}

impl<'global_config, Static: StaticStorage, Dynamic: DynamicStorage<DynamicConfig>>
    ServiceState<'global_config, Static, Dynamic>
{
    pub fn new(
        static_config: StaticConfig,
        global_config: &'global_config global_config::Entries,
        dynamic_storage: Dynamic,
        static_storage: Static,
    ) -> Self {
        let new_self = Self {
            static_config,
            global_config,
            dynamic_storage,
            static_storage,
        };
        trace!(from new_self, "open service");
        new_self
    }
}

impl<'global_config, Static: StaticStorage, Dynamic: DynamicStorage<DynamicConfig>> Drop
    for ServiceState<'global_config, Static, Dynamic>
{
    fn drop(&mut self) {
        match self.dynamic_storage.get().decrement_reference_counter() {
            DecrementReferenceCounterResult::HasOwners => {
                trace!(from self, "close service");
            }
            DecrementReferenceCounterResult::NoMoreOwners => {
                self.static_storage.acquire_ownership();
                self.dynamic_storage.acquire_ownership();
                trace!(from self, "close and remove service");
            }
        }
    }
}

pub trait Service: Sized {
    type Type<'a>: Details<'a>;

    fn new(name: &ServiceName) -> Builder<Self> {
        Builder::new(name)
    }
}

pub trait Details<'global_config>: Debug + Sized {
    type ServiceNameHasher: Hash;
    type StaticStorage: StaticStorage;
    type ConfigSerializer: Serialize;
    type DynamicStorage: DynamicStorage<DynamicConfig>;
    type SharedMemory: SharedMemory<PoolAllocator>;
    type Connection: ZeroCopyConnection;
    type Event: Event<EventId>;

    fn from_state(
        state: ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage>,
    ) -> Self;

    fn state(&self) -> &ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage>;

    fn state_mut(
        &mut self,
    ) -> &mut ServiceState<'global_config, Self::StaticStorage, Self::DynamicStorage>;

    fn does_exist(service_name: &ServiceName) -> Result<bool, ServiceDoesExistError> {
        Self::does_exist_from_config(service_name, global_config::Config::get_global_config())
    }

    fn does_exist_from_config(
        service_name: &ServiceName,
        config: &'global_config global_config::Config,
    ) -> Result<bool, ServiceDoesExistError> {
        let msg = format!("Unable to verify if \"{}\" exists", service_name);
        let origin = "Service::does_exist_from_config()";
        let static_storage_config = static_config_storage_config::<Self>(config.get());

        let services = fail!(from origin,
                 when <Self::StaticStorage as NamedConceptMgmt>::list_cfg(&static_storage_config),
                 map NamedConceptListError::InsufficientPermissions => ServiceDoesExistError::InsufficientPermissions,
                 unmatched ServiceDoesExistError::InternalError,
                 "{} due to a failure while collecting all active services for config: {:?}", msg, config);

        for service_storage in services {
            let reader =
                match <<Self::StaticStorage as StaticStorage>::Builder as NamedConceptBuilder<
                    Self::StaticStorage,
                >>::new(&service_storage)
                .config(&static_storage_config.clone())
                .has_ownership(false)
                .open()
                {
                    Ok(reader) => reader,
                    Err(e) => {
                        warn!(from origin, "Unable to open service static info \"{}\" for reading ({:?}). Maybe unable to determin if the service \"{}\" exists.",
                            service_storage, e, service_name);
                        continue;
                    }
                };

            let mut content = String::from_utf8(vec![b' '; reader.len() as usize]).unwrap();
            if let Err(e) = reader.read(unsafe { content.as_mut_vec().as_mut_slice() }) {
                warn!(from origin, "Unable to read service static info \"{}\" - error ({:?}). Maybe unable to determin if the service \"{}\" exists.",
                            service_storage, e, service_name);
            }

            let service_config = match Self::ConfigSerializer::deserialize::<StaticConfig>(unsafe {
                content.as_mut_vec()
            }) {
                Ok(service_config) => service_config,
                Err(e) => {
                    warn!(from origin, "Unable to deserialize service static info \"{}\" - error ({:?}). Maybe unable to determin if the service \"{}\" exists.",
                            service_storage, e, service_name);
                    continue;
                }
            };

            if service_storage.as_bytes() != service_config.uuid().as_bytes() {
                warn!(from origin, "Detected service {:?} with an inconsistent hash of {} when acquiring services according to config {:?}",
                    service_config, service_storage, config);
                continue;
            }

            if service_config.service_name() == service_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn list() -> Result<Vec<StaticConfig>, ServiceListError> {
        Self::list_from_config(global_config::Config::get_global_config())
    }

    fn list_from_config(
        config: &'global_config global_config::Config,
    ) -> Result<Vec<StaticConfig>, ServiceListError> {
        let msg = "Unable to list all services";
        let origin = "Service::list_from_config()";
        let static_storage_config = static_config_storage_config::<Self>(config.get());

        let services = fail!(from origin,
                when <Self::StaticStorage as NamedConceptMgmt>::list_cfg(&static_storage_config),
                map NamedConceptListError::InsufficientPermissions => ServiceListError::InsufficientPermissions,
                unmatched ServiceListError::InternalError,
                "{} due to a failure while collecting all active services for config: {:?}", msg, config);

        let mut service_vec = vec![];
        for service_storage in services {
            let reader =
                match <<Self::StaticStorage as StaticStorage>::Builder as NamedConceptBuilder<
                    Self::StaticStorage,
                >>::new(&service_storage)
                .config(&static_storage_config.clone())
                .has_ownership(false)
                .open()
                {
                    Ok(reader) => reader,
                    Err(e) => {
                        warn!(from origin, "Unable to acquire a list of all service since the static service info \"{}\" could not be opened for reading ({:?}).",
                           service_storage, e );
                        continue;
                    }
                };

            let mut content = String::from_utf8(vec![b' '; reader.len() as usize]).unwrap();
            if let Err(e) = reader.read(unsafe { content.as_mut_vec().as_mut_slice() }) {
                warn!(from origin, "Unable to acquire a list of all service since the static service info \"{}\" could not be read ({:?}).",
                           service_storage, e );
                continue;
            }

            let service_config = match Self::ConfigSerializer::deserialize::<StaticConfig>(unsafe {
                content.as_mut_vec()
            }) {
                Ok(service_config) => service_config,
                Err(e) => {
                    warn!(from origin, "Unable to acquire a list of all service since the static service info \"{}\" could not be deserialized ({:?}).",
                       service_storage, e );
                    continue;
                }
            };

            if service_storage.as_bytes() != service_config.uuid().as_bytes() {
                warn!(from origin, "Detected service {:?} with an inconsistent hash of {} when acquiring services according to config {:?}",
                    service_config, service_storage, config);
                continue;
            }

            service_vec.push(service_config);
        }

        Ok(service_vec)
    }
}
