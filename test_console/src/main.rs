// Using your custom legacy parser
// use common::configuration_service::ConfigurationService;
// use common::connection_provider::ConnectionProviderPostgres;
// Using the modern TOML parser
// use common::configuration_service::ModernConfig;
use common::log_sink_db::LogSinkDb;
use std::error::Error;



// ==========================================
// 3. Test Main Entry Point
// ==========================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Point the database logger to "ITFORS1011" using the default AppConfig.toml
    let mut logger = LogSinkDb::new("driope", "ITFORS1011", "default").await?;

    println!("LogSinkDb successfully created for server ITFORS1011!");
    logger.trace(
        "AAA", // row_nature: &str,    // 3 chars {o,t,c} or {INF,DBG,..}
        "1", // stack_depth: &str,    // call stack depth
        "main", // function_name: &str,  // calling method name
        "message content" //  log_content: &str,    // message content
    ).await?;
    
    // Clean up and close connection
    logger.dispose().await;

    Ok(())
    
}

/*  cantina
fn main() {



    let config = ModernConfig::load("default")?;

    // Choose your server target parametrically
    let target_server = "ITFORS1011";

    if let Some(db_conf) = config.get_db_config(target_server) {
        println!("Connecting to {} at URL: {}", target_server, db_conf.url);
        // Initialize your database connection here using db_conf.url, db_conf.user, etc.
    } else {
        println!("Error: Configuration for server '{}' not found!", target_server);
    }
    //--------------------------------
// Pass the desired server name and configuration path ("default" looks for ./AppConfig.toml)
    let mut provider = ConnectionProviderPostgres::new( target_server, "default").await?;

    if let Some(conn) = provider.get_connection() {
        // Run your queries here using `conn`
        println!("Successfully acquired mutable connection handle!");
    }

    provider.close_connection().await;


    //------------LogSinkDb
    // Initialize our LogSinkDb with a target table name and properties file
    let mut logger = common::log_sink_db::LogSinkDb::new(target_server, "amadriadi").await?;

    // Simulate a fast numerical loop calling trace repeatedly using the same sticky connection
    for i in 1..=3 {
        let message = format!("Iteration step value: {}", i * 100);
        logger.trace("INF", "1", "main_loop", &message).await?;
        println!("Logged iteration {}", i);
    }
    let legacy_config = ConfigurationService::new("default");
    if legacy_config.read_is_healthy_constructed() {
        println!("Legacy config loaded successfully!");
    }

    if let Ok(modern_config) = ModernConfig::load("default") {
        println!("Modern TOML verbosity: {}", modern_config.verbosity);
    }
}

*/