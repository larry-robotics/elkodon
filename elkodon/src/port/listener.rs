use elkodon_bb_lock_free::mpmc::unique_index_set::UniqueIndex;
use elkodon_bb_log::fail;
use elkodon_cal::dynamic_storage::DynamicStorage;
use elkodon_cal::event::{ListenerBuilder, ListenerWaitError};
use elkodon_cal::named_concept::NamedConceptBuilder;

use crate::service::event_concept_name;
use crate::{port::port_identifiers::UniqueListenerId, service};
use std::{marker::PhantomData, time::Duration};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ListenerCreateError {
    ExceedsMaxSupportedListeners,
    ResourceCreationFailed,
}

#[derive(Debug)]
pub struct Listener<'a, 'global_config: 'a, Service: service::Details<'global_config>> {
    _dynamic_config_guard: Option<UniqueIndex<'a>>,
    listener: <Service::Event as elkodon_cal::event::Event<u64>>::Listener,
    _phantom_a: PhantomData<&'a Service>,
    _phantom_b: PhantomData<&'global_config ()>,
}

impl<'a, 'global_config: 'a, Service: service::Details<'global_config>>
    Listener<'a, 'global_config, Service>
{
    pub(crate) fn new(service: &'a Service) -> Result<Self, ListenerCreateError> {
        let msg = "Failed to create listener";
        let origin = "Listener::new()";
        let port_id = UniqueListenerId::new();

        let event_name = event_concept_name(&port_id);
        let listener = fail!(from origin,
                             when <Service::Event as elkodon_cal::event::Event<u64>>::ListenerBuilder::new(&event_name).create(),
                             with ListenerCreateError::ResourceCreationFailed,
                             "{} since the underlying event concept \"{}\" could not be created.", msg, event_name);

        let mut new_self = Self {
            _dynamic_config_guard: None,
            listener,
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

    pub fn try_wait<F: FnMut(u64) -> bool>(
        &self,
        mut notification_received_callback: F,
    ) -> Result<u64, ListenerWaitError> {
        use elkodon_cal::event::Listener;
        let mut number_of_events = 0;
        while let Some(id) = fail!(from self,
                when self.listener.try_wait(),
                "Failed to try_wait on Listener port since the underlying Listener concept failed.")
        {
            number_of_events += 1;
            if !notification_received_callback(id) {
                break;
            }
        }

        Ok(number_of_events)
    }

    pub fn timed_wait<F: FnMut(u64) -> bool>(
        &self,
        mut notification_received_callback: F,
        timeout: Duration,
    ) -> Result<u64, ListenerWaitError> {
        use elkodon_cal::event::Listener;
        if let Some(id) = fail!(from self,
            when self.listener.timed_wait(timeout),
            "Failed to timed_wait with timeout {:?} on Listener port since the underlying Listener concept failed.", timeout)
        {
            if notification_received_callback(id) {
                return Ok(self.try_wait(notification_received_callback)? + 1);
            }
        }

        Ok(1)
    }

    pub fn blocking_wait<F: FnMut(u64) -> bool>(
        &self,
        mut notification_received_callback: F,
    ) -> Result<u64, ListenerWaitError> {
        use elkodon_cal::event::Listener;
        if let Some(id) = fail!(from self,
            when self.listener.blocking_wait(),
            "Failed to blocking_wait on Listener port since the underlying Listener concept failed.")
        {
            if notification_received_callback(id) {
                return Ok(self.try_wait(notification_received_callback)? + 1);
            }
        }

        Ok(1)
    }
}
