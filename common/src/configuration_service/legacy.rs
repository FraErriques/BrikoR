use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ConfigurationService {
    key_val_map: HashMap<String, String>,
    key_all_together: Vec<String>,
    is_healthy: bool,
    why_not_healthy: String,
}

impl ConfigurationService {
    pub fn new(config_path: &str) -> Self {
        let path = if config_path == "default" {
            "./AppConfig.txt"
        } else {
            config_path
        };

        let mut service = Self {
            key_val_map: HashMap::new(),
            key_all_together: Vec::new(),
            is_healthy: false,
            why_not_healthy: String::new(),
        };

        if !Path::new(path).exists() {
            service.why_not_healthy = "ConfigFile does not exist on FileSystem.".to_string();
            return service;
        }

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                service.why_not_healthy = "ConfigFile failed to open for read.".to_string();
                return service;
            }
        };

        let tokens: Vec<&str> = contents.split_whitespace().collect();
        let total_tokens = tokens.len();

        if total_tokens % 2 == 0 {
            service.why_not_healthy = "ConfigFile tokens count must be odd (n + 1 + n).".to_string();
            return service;
        }

        let n = total_tokens / 2;

        if tokens[n] != "#" {
            service.why_not_healthy = "Missing '#' separator token between keys and values.".to_string();
            return service;
        }

        for i in 0..n {
            let key = tokens[i].to_string();
            let val = tokens[i + n + 1].to_string();
            
            service.key_all_together.push(key.clone());
            service.key_val_map.insert(key, val);
        }

        service.is_healthy = true;
        service.why_not_healthy = "healthy".to_string();
        service
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        if self.is_healthy {
            self.key_val_map.get(key)
        } else {
            None
        }
    }

    pub fn get_all_keys(&self) -> &[String] {
        &self.key_all_together
    }

    pub fn read_is_healthy_constructed(&self) -> bool {
        self.is_healthy
    }
}