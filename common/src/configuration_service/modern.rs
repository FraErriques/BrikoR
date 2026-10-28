use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModernConfig {
    pub log_full_path_prefix: Option<String>,
    pub semaphore: Option<String>,
    pub verbosity: Option<i32>,
    // Maps server name (e.g., "ITFORS1011", "Cantor") to its DbConfig
    #[serde(default)]
    pub db: HashMap<String, DbConfig>,
}

impl ModernConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = if path == "default" {
            "./AppConfig.toml"
        } else {
            path
        };

        if !Path::new(config_path).exists() {
            return Err("Modern config file does not exist".into());
        }

        let contents = fs::read_to_string(config_path)?;
        let config: ModernConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Helper to fetch database configuration dynamically for any given server name
    pub fn get_db_config(&self, server_name: &str) -> Option<&DbConfig> {
        self.db.get(server_name)
    }
}