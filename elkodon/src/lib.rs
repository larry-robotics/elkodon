pub mod global_config;
pub mod message;
pub mod port;
pub mod sample;
pub mod sample_mut;
pub mod service;

pub mod prelude {
    pub use crate::port::event_id::EventId;
    pub use crate::service::{
        process_local, service_name::ServiceName, zero_copy, Details, Service,
    };
    pub use elkodon_bb_container::semantic_string::SemanticString;
}
