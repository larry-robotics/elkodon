use elkodon_bb_log::fatal_panic;
use elkodon_bb_posix::unique_system_id::UniqueSystemId;

macro_rules! generate_id {
    { $id_name:ident } => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $id_name(pub(crate) UniqueSystemId);

        impl Default for $id_name {
            fn default() -> Self {
                Self(
                    fatal_panic!(from format!("{}::new()", stringify!($id_name)), when UniqueSystemId::new(),
                        "Unable to generate required {}!", stringify!($id_name)),
                )
            }
        }

        impl $id_name {
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}

generate_id! { UniquePublisherId }
generate_id! { UniqueSubscriberId }
generate_id! { UniqueNotifierId }
generate_id! { UniqueListenerId }
