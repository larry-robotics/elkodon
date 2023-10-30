use elkodon_bb_elementary::relocatable_container::RelocatableContainer;
use elkodon_bb_lock_free::mpmc::{container::*, unique_index_set::UniqueIndex};
use elkodon_bb_log::fatal_panic;
use elkodon_bb_memory::bump_allocator::BumpAllocator;

use crate::port::port_identifiers::{UniquePublisherId, UniqueSubscriberId};

#[derive(Debug, Clone, Copy)]
pub struct DynamicConfigSettings {
    pub number_of_subscribers: usize,
    pub number_of_publishers: usize,
}

#[derive(Debug)]
pub struct DynamicConfig {
    pub(crate) subscribers: Container<UniqueSubscriberId>,
    pub(crate) publishers: Container<UniquePublisherId>,
}

impl DynamicConfig {
    pub fn new(config: &DynamicConfigSettings) -> Self {
        Self {
            subscribers: unsafe { Container::new_uninit(config.number_of_subscribers) },
            publishers: unsafe { Container::new_uninit(config.number_of_publishers) },
        }
    }

    pub(crate) unsafe fn init(&self, allocator: &BumpAllocator) {
        fatal_panic!(from "publish_subscribe::DynamicConfig::init",
            when self.subscribers.init(allocator),
            "This should never happen! Unable to initialize subscriber port id container.");
        fatal_panic!(from "publish_subscribe::DynamicConfig::init",
            when self.publishers.init(allocator),
            "This should never happen! Unable to initialize publisher port id container.");
    }

    pub fn memory_size(config: &DynamicConfigSettings) -> usize {
        Container::<UniqueSubscriberId>::memory_size(config.number_of_subscribers)
            + Container::<UniquePublisherId>::memory_size(config.number_of_publishers)
    }

    pub fn number_of_publishers(&self) -> usize {
        self.publishers.len()
    }

    pub fn number_of_subscribers(&self) -> usize {
        self.subscribers.len()
    }

    pub fn number_of_supported_publishers(&self) -> usize {
        self.publishers.capacity()
    }

    pub fn number_of_supported_subscribers(&self) -> usize {
        self.subscribers.capacity()
    }

    pub fn add_subscriber_id(&self, id: UniqueSubscriberId) -> Option<UniqueIndex> {
        unsafe { self.subscribers.add(id) }
    }

    pub fn add_publisher_id(&self, id: UniquePublisherId) -> Option<UniqueIndex> {
        unsafe { self.publishers.add(id) }
    }
}
