use elkodon_bb_elementary::relocatable_container::RelocatableContainer;
use elkodon_bb_lock_free::mpmc::{container::*, unique_index_set::UniqueIndex};
use elkodon_bb_log::fatal_panic;
use elkodon_bb_memory::bump_allocator::BumpAllocator;

use crate::port::port_identifiers::{UniqueListenerId, UniqueNotifierId};

#[derive(Debug, Clone, Copy)]
pub struct DynamicConfigSettings {
    pub number_of_listeners: usize,
    pub number_of_notifiers: usize,
}

#[derive(Debug)]
pub struct DynamicConfig {
    pub(crate) listeners: Container<UniqueListenerId>,
    pub(crate) notifiers: Container<UniqueNotifierId>,
}

impl DynamicConfig {
    pub fn new(config: &DynamicConfigSettings) -> Self {
        Self {
            listeners: unsafe { Container::new_uninit(config.number_of_listeners) },
            notifiers: unsafe { Container::new_uninit(config.number_of_notifiers) },
        }
    }

    pub(crate) unsafe fn init(&self, allocator: &BumpAllocator) {
        fatal_panic!(from "event::DynamicConfig::init",
            when self.listeners.init(allocator),
            "This should never happen! Unable to initialize listener port id container.");
        fatal_panic!(from "event::DynamicConfig::init",
            when self.notifiers.init(allocator),
            "This should never happen! Unable to initialize notifier port id container.");
    }

    pub fn memory_size(config: &DynamicConfigSettings) -> usize {
        Container::<UniqueListenerId>::memory_size(config.number_of_listeners)
            + Container::<UniqueNotifierId>::memory_size(config.number_of_notifiers)
    }

    pub fn number_of_supported_listeners(&self) -> usize {
        self.listeners.capacity()
    }

    pub fn number_of_supported_notifiers(&self) -> usize {
        self.notifiers.capacity()
    }

    pub fn add_listener_id(&self, id: UniqueListenerId) -> Option<UniqueIndex> {
        unsafe { self.listeners.add(id) }
    }

    pub fn add_notifier_id(&self, id: UniqueNotifierId) -> Option<UniqueIndex> {
        unsafe { self.notifiers.add(id) }
    }
}
