use chrono::Local;
use std::error::Error;
use crate::connection_provider::ConnectionProviderPostgres;

// ==========================================
// 2. LogSinkDb Class
// ==========================================
pub struct LogSinkDb {
    provider: ConnectionProviderPostgres,
    log_table_name: String,
}

impl LogSinkDb {
    // Constructor (associated function)
    pub async fn new(table_name: &str, server_name: &str, config_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut provider = ConnectionProviderPostgres::new(server_name, config_path).await?;

        if let Some(_conn) = provider.get_connection() {
            println!("Database connection initialized for table: {}", table_name);
        }

        let log_table_name = table_name.to_string();

        // Call create_log_table(?) once upon initialization
        let create_table_sql = "SELECT create_log_table($1)";
        if let Some(conn) = provider.get_connection() {
            sqlx::query(create_table_sql)
                .bind(&log_table_name)
                .execute(conn)
                .await?;
            println!("-> Checked/Created log table: {}", log_table_name);
        } else {
            return Err("Failed to obtain active connection for table creation.".into());
        }        

        Ok(Self {
            provider,
            log_table_name: table_name.to_string(),
        })
    }

    // Trace method optimized for numerical analysis loops
    pub async fn trace(
        &mut self,
        row_nature: &str,    // 3 chars {o,t,c} or {INF,DBG,..}
        stack_depth: &str,    // call stack depth
        function_name: &str,  // calling method name
        log_content: &str,    // message content
    ) -> Result<(), Box<dyn Error>> {
        let trace_sql = "SELECT trace($1, $2, $3, $4, $5, $6)";
        let current_formatted_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if let Some(conn) = self.provider.get_connection() {
            sqlx::query(trace_sql)
                .bind(&self.log_table_name)
                .bind(&current_formatted_date)
                .bind(row_nature)
                .bind(stack_depth)
                .bind(function_name)
                .bind(log_content)
                .execute(conn)
                .await?;
        } else {
            println!("Failed to write trace log: Connection is closed or unavailable.");
        }

        Ok(())
    }

    // Explicit dispose method
    pub async fn dispose(&mut self) {
        self.provider.close_connection().await;
    }
}