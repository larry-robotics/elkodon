/// Factory to create the endpoints of
/// [`MessagingPattern::Event`](crate::service::messaging_pattern::MessagingPattern::Event) based
/// communication and to acquire static and dynamic service information
pub mod event;

/// Factory to create a [`crate::port::listener::Listener`]
pub mod listener;

/// Factory to create a [`crate::port::notifier::Notifier`]
pub mod notifier;

/// Factory to create the endpoints of
/// [`MessagingPattern::PublishSubscribe`](crate::service::messaging_pattern::MessagingPattern::PublishSubscribe) based
/// communication and to acquire static and dynamic service information
pub mod publish_subscribe;

/// Factory to create a [`crate::port::publisher::Publisher`]
pub mod publisher;

/// Factory to create a [`crate::port::subscriber::Subscriber`]
pub mod subscriber;
