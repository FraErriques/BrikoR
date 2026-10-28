use crate::configuration_service::ModernConfig;
use sqlx::{Connection, PgConnection};
use std::error::Error;
use std::io;

// ==========================================
// 1. Parametric Connection Provider
// ==========================================
pub struct ConnectionProviderPostgres {
    connection: Option<PgConnection>,
    server_name: String,
}

impl ConnectionProviderPostgres {
    /// Creates a new connection by loading server credentials parametrically from AppConfig.toml
    pub async fn new(server_name: &str, config_path: &str) -> Result<Self, Box<dyn Error>> {
        // Load the modern configuration using your configuration service
        let config = ModernConfig::load(config_path)?;

        // Look up the specific server configuration dynamically
        let db_conf = config.get_db_config(server_name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Database configuration for server '{}' not found in config", server_name),
            )
        })?;

        // Establish the asynchronous PostgreSQL connection using the URL from TOML
        match PgConnection::connect(&db_conf.url).await {
            Ok(conn) => {
                println!("Connection to database {} opened successfully", server_name);
                Ok(Self {
                    connection: Some(conn),
                    server_name: server_name.to_string(),
                })
            }
            Err(e) => {
                println!("Failed to connect to {}: {}", server_name, e);
                Err(Box::new(e))
            }
        }
    }

    pub fn get_connection(&mut self) -> Option<&mut PgConnection> {
        self.connection.as_mut()
    }

    pub async fn close_connection(&mut self) {
        if let Some(conn) = self.connection.take() {
            match conn.close().await {
                Ok(_) => {
                    println!("Connection to database {} closed successfully", self.server_name);
                }
                Err(e) => {
                    println!("\nError closing connection for {}: {}", self.server_name, e);
                }
            }
        }
    }
}