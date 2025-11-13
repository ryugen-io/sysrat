use super::models::{Config, ConfigFile};
use super::scanner::scan_directory;
use std::collections::HashMap;
use std::sync::Arc;

/// Global application state holding the configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    files_by_name: HashMap<String, ConfigFile>,
}

impl AppConfig {
    /// Load configuration from file
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path();

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file {}: {}", config_path, e))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;

        // Build hashmap for fast lookups
        let mut files_by_name = HashMap::new();

        // Add individual files
        for file in config.files {
            // Validate name ends with .conf or .toml
            if !file.name.ends_with(".conf") && !file.name.ends_with(".toml") {
                return Err(format!(
                    "File name must end with .conf or .toml: {}",
                    file.name
                ));
            }
            files_by_name.insert(file.name.clone(), file);
        }

        // Scan directories and add found files
        for dir_config in config.directories {
            match scan_directory(&dir_config) {
                Ok(files) => {
                    for file in files {
                        files_by_name.insert(file.name.clone(), file);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to scan directory {}: {}",
                        dir_config.name, e
                    );
                }
            }
        }

        Ok(AppConfig { files_by_name })
    }

    /// Get all file names
    pub fn list_files(&self) -> Vec<String> {
        let mut names: Vec<_> = self.files_by_name.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get config for a specific file
    pub fn get_file(&self, name: &str) -> Option<&ConfigFile> {
        self.files_by_name.get(name)
    }

    /// Get the config file path
    fn config_path() -> String {
        std::env::var("CONFIG_MANAGER_CONFIG").unwrap_or_else(|_| "config-manager.toml".to_string())
    }
}

/// Shared application state
pub type SharedConfig = Arc<AppConfig>;
