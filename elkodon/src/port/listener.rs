use elkodon_bb_lock_free::mpmc::unique_index_set::UniqueIndex;
use elkodon_bb_log::fail;
use elkodon_cal::dynamic_storage::DynamicStorage;
use elkodon_cal::event::{ListenerBuilder, ListenerWaitError};
use elkodon_cal::named_concept::NamedConceptBuilder;

use crate::service::event_concept_name;
use crate::{port::port_identifiers::UniqueListenerId, service};
use std::{marker::PhantomData, time::Duration};

use super::event_id::EventId;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ListenerCreateError {
    ExceedsMaxSupportedListeners,
    ResourceCreationFailed,
}

impl std::fmt::Display for ListenerCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}::{:?}", std::stringify!(Self), self)
    }
}

impl std::error::Error for ListenerCreateError {}

#[derive(Debug)]
pub struct Listener<'a, 'config: 'a, Service: service::Details<'config>> {
    _dynamic_config_guard: Option<UniqueIndex<'a>>,
    listener: <Service::Event as elkodon_cal::event::Event<EventId>>::Listener,
    cache: Vec<EventId>,
    _phantom_a: PhantomData<&'a Service>,
    _phantom_b: PhantomData<&'config ()>,
}

impl<'a, 'config: 'a, Service: service::Details<'config>> Listener<'a, 'config, Service> {
    pub(crate) fn new(service: &'a Service) -> Result<Self, ListenerCreateError> {
        let msg = "Failed to create listener";
        let origin = "Listener::new()";
        let port_id = UniqueListenerId::new();

        let event_name = event_concept_name(&port_id);
        let listener = fail!(from origin,
                             when <Service::Event as elkodon_cal::event::Event<EventId>>::ListenerBuilder::new(&event_name).create(),
                             with ListenerCreateError::ResourceCreationFailed,
                             "{} since the underlying event concept \"{}\" could not be created.", msg, event_name);

        let mut new_self = Self {
            _dynamic_config_guard: None,
            listener,
            cache: vec![],
            _phantom_a: PhantomData,
            _phantom_b: PhantomData,
        };

        // !MUST! be the last task otherwise a listener is added to the dynamic config without
        // the creation of all required channels
        new_self._dynamic_config_guard = Some(
            match service
                .state()
                .dynamic_storage
                .get()
                .event()
                .add_listener_id(port_id)
            {
                Some(unique_index) => unique_index,
                None => {
                    fail!(from origin, with ListenerCreateError::ExceedsMaxSupportedListeners,
                                 "{} since it would exceed the maximum supported amount of listeners of {}.",
                                 msg, service.state().static_config.event().max_listeners);
                }
            },
        );

        Ok(new_self)
    }

    fn fill_cache(&mut self) -> Result<(), ListenerWaitError> {
        use elkodon_cal::event::Listener;
        while let Some(id) = fail!(from self,
                when self.listener.try_wait(),
                "Failed to try_wait on Listener port since the underlying Listener concept failed.")
        {
            self.cache.push(id);
        }

        Ok(())
    }

    pub fn cache(&self) -> &[EventId] {
        &self.cache
    }

    pub fn try_wait(&mut self) -> Result<&[EventId], ListenerWaitError> {
        self.cache.clear();
        self.fill_cache()?;

        Ok(self.cache())
    }

    pub fn timed_wait(&mut self, timeout: Duration) -> Result<&[EventId], ListenerWaitError> {
        use elkodon_cal::event::Listener;
        self.cache.clear();

        if let Some(id) = fail!(from self,
            when self.listener.timed_wait(timeout),
            "Failed to timed_wait with timeout {:?} on Listener port since the underlying Listener concept failed.", timeout)
        {
            self.cache.push(id);
            self.fill_cache()?;
        }

        Ok(self.cache())
    }

    pub fn blocking_wait(&mut self) -> Result<&[EventId], ListenerWaitError> {
        use elkodon_cal::event::Listener;
        self.cache.clear();

        if let Some(id) = fail!(from self,
            when self.listener.blocking_wait(),
            "Failed to blocking_wait on Listener port since the underlying Listener concept failed.")
        {
            self.cache.push(id);
            self.fill_cache()?;
        }

        Ok(self.cache())
    }
}
