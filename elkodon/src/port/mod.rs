use std::fmt::Debug;

use tiny_fn::tiny_fn;

pub(crate) mod details;
pub mod listener;
pub mod notifier;
pub mod port_identifiers;
pub mod publisher;
pub mod subscriber;

use crate::port::port_identifiers::*;
use crate::service;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DegrationAction {
    Ignore,
    Warn,
    Fail,
}

tiny_fn! {
    pub struct DegrationCallback = Fn(service: service::static_config::StaticConfig, publisher_id: UniquePublisherId, subscriber_id: UniqueSubscriberId) -> DegrationAction;
}

impl<'a> Debug for DegrationCallback<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
