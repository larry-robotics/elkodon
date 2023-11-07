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

#[cfg(target_os = "windows")]
pub const DEFAULT_CONFIG_FILE: FilePath =
    unsafe { FilePath::new_unchecked(b"config\\elkodon_win.toml") };

#[cfg(not(target_os = "windows"))]
pub const DEFAULT_CONFIG_FILE: FilePath =
    unsafe { FilePath::new_unchecked(b"config/elkodon.toml") };

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Service {
    pub directory: String,
    pub publisher_data_segment_suffix: String,
    pub static_config_storage_suffix: String,
    pub dynamic_config_storage_suffix: String,
    pub creation_timeout: Duration,
    pub connection_suffix: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Global {
    pub root_path: String,
    pub service: Service,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Defaults {
    pub publish_subscribe: PublishSubscribe,
    pub event: Event,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishSubscribe {
    pub max_subscribers: usize,
    pub max_publishers: usize,
    pub subscriber_buffer_size: usize,
    pub subscriber_max_borrowed_samples: usize,
    pub publisher_max_loaned_samples: usize,
    pub publisher_history_size: usize,
    pub enable_safe_overflow: bool,
    pub unable_to_deliver_strategy: UnableToDeliverStrategy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub max_listeners: usize,
    pub max_notifiers: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entries {
    pub global: Global,
    pub defaults: Defaults,
}

impl Default for Entries {
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
                    subscriber_buffer_size: 2,
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

#[derive(Debug, Default)]
pub struct Config {
    entries: Entries,
}

static ELKODON_CONFIG: LazySingleton<Config> = LazySingleton::<Config>::new();

impl Config {
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
            Ok(v) => new_config.entries = v,
            Err(e) => {
                fail!(from new_config, with ConfigCreationError::UnableToDeserializeContents,
                                "{} since the contents could not be deserialized ({}).", msg, e);
            }
        }

        trace!(from new_config, "Loaded.");
        Ok(new_config)
    }

    pub fn from_entries(entries: &Entries) -> Self {
        Self {
            entries: entries.clone(),
        }
    }

    pub fn get(&self) -> &Entries {
        &self.entries
    }

    pub fn setup_from_file(config_file: &FilePath) -> Result<&'static Config, ConfigCreationError> {
        if ELKODON_CONFIG.is_initialized() {
            return Ok(ELKODON_CONFIG.get());
        }

        if !ELKODON_CONFIG.set_value(Config::from_file(config_file)?) {
            warn!(
                from ELKODON_CONFIG.get(),
                "Configuration already loaded and set up, cannot load another one."
            );
            return Ok(ELKODON_CONFIG.get());
        }

        trace!(from ELKODON_CONFIG.get(), "Set as global config.");
        Ok(ELKODON_CONFIG.get())
    }

    pub fn get_global_config() -> &'static Config {
        if !ELKODON_CONFIG.is_initialized()
            && Config::setup_from_file(&DEFAULT_CONFIG_FILE).is_err()
        {
            warn!(from "Config::get_global_config()", "Unable to load default config file, populate config with default values.");
            ELKODON_CONFIG.set_value(Config::default());
        }

        ELKODON_CONFIG.get()
    }
}
