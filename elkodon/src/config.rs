//! # Examples
//!
//! ## Publish-Subscriber
//!
//! ```
//! use elkodon::prelude::*;
//! use elkodon::config::Config;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
//!
//! // create a default config and override some entries
//! let mut custom_config = Config::default();
//! custom_config.defaults.publish_subscribe.max_publishers = 5;
//! custom_config.global.service.directory = "another_service_dir".to_string();
//!
//! let service = zero_copy::Service::new(&service_name)
//!     .publish_subscribe_with_custom_config(&custom_config)
//!     .open_or_create::<u64>()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Event
//!
//! ```
//! use elkodon::prelude::*;
//! use elkodon::config::Config;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let event_name = ServiceName::new(b"MyEventName")?;
//!
//! // create a default config and override some entries
//! let mut custom_config = Config::default();
//! custom_config.defaults.event.max_notifiers = 5;
//! custom_config.global.service.directory = "custom_service_dir".to_string();
//!
//! let event = zero_copy::Service::new(&event_name)
//!     .event_with_custom_config(&custom_config)
//!     .open_or_create()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Set Global Config From Custom File
//!
//! The [`crate::config::Config::setup_global_config_from_file()`] call must be the first
//! call in the system. If another
//! instance accesses the global config, it will be loaded with default values and can no longer
//! be overridden with new values from a custom file.
//!
//! ```no_run
//! use elkodon::config::Config;
//! use elkodon_bb_system_types::file_path::FilePath;
//! use elkodon_bb_container::semantic_string::SemanticString;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! Config::setup_global_config_from_file(
//!     &FilePath::new(b"my/custom/config/file.toml")?)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Generate Config From Custom File
//!
//! ```no_run
//! use elkodon::config::Config;
//! use elkodon_bb_system_types::file_path::FilePath;
//! use elkodon_bb_container::semantic_string::SemanticString;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let custom_config = Config::from_file(
//!     &FilePath::new(b"my/custom/config/file.toml")?)?;
//! # Ok(())
//! # }
//! ```

use elkodon_bb_container::byte_string::FixedSizeByteString;
use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_elementary::lazy_singleton::*;
use elkodon_bb_posix::{file::FileBuilder, shared_memory::AccessMode};
use elkodon_bb_system_types::file_path::FilePath;
use elkodon_bb_system_types::path::Path;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use elkodon_bb_log::{fail, trace, warn};

use crate::service::port_factory::publisher::UnableToDeliverStrategy;

/// Path to the default config file
#[cfg(target_os = "windows")]
pub const DEFAULT_CONFIG_FILE: FilePath =
    unsafe { FilePath::new_unchecked(b"config\\elkodon_win.toml") };

/// Path to the default config file
#[cfg(not(target_os = "windows"))]
pub const DEFAULT_CONFIG_FILE: FilePath =
    unsafe { FilePath::new_unchecked(b"config/elkodon.toml") };

/// Failures occurring while creating a new [`Config`] object with [`Config::from_file()`] or
/// [`Config::setup_global_config_from_file()`]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum ConfigCreationError {
    FailedToOpenConfigFile,
    FailedToReadConfigFileContents,
    UnableToDeserializeContents,
}

impl std::fmt::Display for ConfigCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}::{:?}", std::stringify!(Self), self)
    }
}

impl std::error::Error for ConfigCreationError {}

/// All configurable settings of a [`crate::service::Service`].
#[non_exhaustive]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Service {
    /// The directory in which all service files are stored
    pub directory: String,
    /// The suffix of the publishers data segment
    pub publisher_data_segment_suffix: String,
    /// The suffix of the static config file
    pub static_config_storage_suffix: String,
    /// The suffix of the dynamic config file
    pub dynamic_config_storage_suffix: String,
    /// Defines the time of how long another process will wait until the service creation is
    /// finalized
    pub creation_timeout: Duration,
    /// The suffix of a one-to-one connection
    pub connection_suffix: String,
}

/// The global settings
#[non_exhaustive]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Global {
    /// The path under which all other directories or files will be created
    pub root_path: String,
    /// [`crate::service::Service`] settings
    pub service: Service,
}

impl Global {
    pub fn get_absolute_service_dir(&self) -> Path {
        let mut path = Path::new(self.root_path.as_bytes()).unwrap();
        path.add_path_entry(
            &FixedSizeByteString::from_bytes(self.service.directory.as_bytes()).unwrap(),
        )
        .unwrap();
        path
    }
}

/// Default settings. These values are used when the user in the code does not specify anything
/// else.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Defaults {
    /// Default settings for the messaging pattern publish-subscribe
    pub publish_subscribe: PublishSubscribe,
    /// Default settings for the messaging pattern event
    pub event: Event,
}

/// Default settings for the publish-subscribe messaging pattern. These settings are used unless
/// the user specifies custom QoS or port settings.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishSubscribe {
    /// The maximum amount of supported [`crate::port::subscriber::Subscriber`]
    pub max_subscribers: usize,
    /// The maximum amount of supported [`crate::port::publisher::Publisher`]
    pub max_publishers: usize,
    /// The maximum buffer size a [`crate::port::subscriber::Subscriber`] can have
    pub subscriber_max_buffer_size: usize,
    /// The maximum amount of [`crate::sample::Sample`]s a [`crate::port::subscriber::Subscriber`] can
    /// hold in parallel.
    pub subscriber_max_borrowed_samples: usize,
    /// The maximum amount of [`crate::sample_mut::SampleMut`]s a [`crate::port::publisher::Publisher`] can
    /// loan in parallel.
    pub publisher_max_loaned_samples: usize,
    /// The maximum history size a [`crate::port::subscriber::Subscriber`] can request from a
    /// [`crate::port::publisher::Publisher`].
    pub publisher_history_size: usize,
    /// Defines how the [`crate::port::subscriber::Subscriber`] buffer behaves when it is
    /// full. When safe overflow is activated, the [`crate::port::publisher::Publisher`] will
    /// replace the oldest [`crate::sample::Sample`] with the newest one.
    pub enable_safe_overflow: bool,
    /// If no safe overflow is activated it defines the deliver strategy of the
    /// [`crate::port::publisher::Publisher`] when the [`crate::port::subscriber::Subscriber`]s
    /// buffer is full.
    pub unable_to_deliver_strategy: UnableToDeliverStrategy,
}

/// Default settings for the event messaging pattern. These settings are used unless
/// the user specifies custom QoS or port settings.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /// The maximum amount of supported [`crate::port::listener::Listener`]
    pub max_listeners: usize,
    /// The maximum amount of supported [`crate::port::notifier::Notifier`]
    pub max_notifiers: usize,
}

/// Represents the configuration that Elkodon will utilize. It is divided into two sections:
/// the [Global] settings, which must align with the Elkodon instance the application intends to
/// join, and the [Defaults] for communication within that Elkodon instance. The user has the
/// flexibility to override both sections.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Global settings for the elkodon instance
    pub global: Global,
    /// Default settings
    pub defaults: Defaults,
}

static ELKODON_CONFIG: LazySingleton<Config> = LazySingleton::<Config>::new();

impl Default for Config {
    fn default() -> Self {
        Self {
            global: Global {
                #[cfg(not(target_os = "windows"))]
                root_path: "/tmp/elkodon/".to_string(),
                #[cfg(target_os = "windows")]
                root_path: "C:\\Windows\\Temp\\elkodon\\".to_string(),
                service: Service {
                    directory: "services".to_string(),
                    publisher_data_segment_suffix: ".publisher_data".to_string(),
                    static_config_storage_suffix: ".service".to_string(),
                    dynamic_config_storage_suffix: ".dynamic".to_string(),
                    creation_timeout: Duration::from_millis(500),
                    connection_suffix: ".connection".to_string(),
                },
            },
            defaults: Defaults {
                publish_subscribe: PublishSubscribe {
                    max_subscribers: 8,
                    max_publishers: 2,
                    publisher_history_size: 1,
                    subscriber_max_buffer_size: 2,
                    subscriber_max_borrowed_samples: 2,
                    publisher_max_loaned_samples: 2,
                    enable_safe_overflow: true,
                    unable_to_deliver_strategy: UnableToDeliverStrategy::Block,
                },
                event: Event {
                    max_listeners: 1,
                    max_notifiers: 16,
                },
            },
        }
    }
}

impl Config {
    /// Loads a configuration from a file. On success it returns a [`Config`] object otherwise a
    /// [`ConfigCreationError`] describing the failure.
    pub fn from_file(config_file: &FilePath) -> Result<Config, ConfigCreationError> {
        let msg = "Failed to create config";
        let mut new_config = Self::default();

        let file = fail!(from new_config, when FileBuilder::new(config_file).open_existing(AccessMode::Read),
                with ConfigCreationError::FailedToOpenConfigFile,
                "{} since the config file could not be opened.", msg);

        let mut contents = String::new();
        fail!(from new_config, when file.read_to_string(&mut contents),
                with ConfigCreationError::FailedToReadConfigFileContents,
                "{} since the config file contents could not be read.", msg);

        match toml::from_str(&contents) {
            Ok(v) => new_config = v,
            Err(e) => {
                fail!(from new_config, with ConfigCreationError::UnableToDeserializeContents,
                                "{} since the contents could not be deserialized ({}).", msg, e);
            }
        }

        trace!(from new_config, "Loaded.");
        Ok(new_config)
    }

    /// Sets up the global configuration from a file. If the global configuration was already setup
    /// it will print a warning and does not load the file. It returns the [`Config`] when the file
    /// could be successfully loaded otherwise a [`ConfigCreationError`] describing the error.
    pub fn setup_global_config_from_file(
        config_file: &FilePath,
    ) -> Result<&'static Config, ConfigCreationError> {
        if ELKODON_CONFIG.is_initialized() {
            return Ok(ELKODON_CONFIG.get());
        }

        if !ELKODON_CONFIG.set_value(Config::from_file(config_file)?) {
            warn!(
                from ELKODON_CONFIG.get(),
                "Configuration already loaded and set up, cannot load another one. This may happen when this function is called from multiple threads."
            );
            return Ok(ELKODON_CONFIG.get());
        }

        trace!(from ELKODON_CONFIG.get(), "Set as global config.");
        Ok(ELKODON_CONFIG.get())
    }

    /// Returns the global configuration. If the global configuration was not
    /// [`Config::setup_global_config_from_file()`] it will load a default config. If
    /// [`Config::setup_global_config_from_file()`]
    /// is called after this function was called, no file will be loaded since the global default
    /// config was already populated.
    pub fn get_global_config() -> &'static Config {
        if !ELKODON_CONFIG.is_initialized()
            && Config::setup_global_config_from_file(&DEFAULT_CONFIG_FILE).is_err()
        {
            warn!(from "Config::get_global_config()", "Unable to load default config file, populate config with default values.");
            ELKODON_CONFIG.set_value(Config::default());
        }

        ELKODON_CONFIG.get()
    }
}
