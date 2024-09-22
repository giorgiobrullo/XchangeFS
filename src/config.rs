use config::{Config, Environment, File as ConfigFile};
use dirs::{data_dir, home_dir};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)] // This will apply default values for missing fields
pub struct AppConfig {
    pub data_dir: PathBuf, // Where to store XchageFS's data configs
    pub listen_addr: Vec<String>, // Where the libp2p node should listen for connections
    pub mount_path: PathBuf, // Where to mount the filesystem that (eventually) will be shared
    pub idle_timeout_secs: u64, // TODO: Ping-specific, to remove
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_data_dir = data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("XchangeFS");
        AppConfig {
            data_dir: default_data_dir,
            listen_addr: vec!["0.0.0.0:0".to_string()],
            mount_path: home_dir().unwrap().join("XchangeFS"),
            idle_timeout_secs: 300,
        }
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize the config with defaults
        let mut settings = Config::builder().set_default("data_dir", AppConfig::default().data_dir.to_str().unwrap())?
                                           .set_default("listen_addr", AppConfig::default().listen_addr.clone())?
                                           .set_default("idle_timeout_secs", AppConfig::default().idle_timeout_secs)?;

        // Optionally load configuration from a file if specified
        if let Some(config_path) = std::env::var("XCHANGEFS__CONFIG").ok() {
            info!("Loading config from file: {}", config_path);
            settings = settings.add_source(ConfigFile::with_name(&config_path));
        } else {
            settings = settings.add_source(ConfigFile::with_name("config").required(false));
        }

        // Load settings from environment variables (if available)
        settings = settings.add_source(
            Environment::with_prefix("XCHANGEFS")
                .separator("__")
                .try_parsing(true)
                .list_separator(",")
                .with_list_parse_key("LISTEN_ADDR")
        );

        // Build the final settings and merge with the defaults
        let settings = settings.build()?;

        // Deserialize into the AppConfig struct
        let config: AppConfig = settings.try_deserialize()?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(
            config.data_dir,
            data_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("XchangeFS")
        );
        assert_eq!(config.listen_addr, vec!["0.0.0.0:8080".to_string()]);
        assert_eq!(config.idle_timeout_secs, 300);
    }
}

