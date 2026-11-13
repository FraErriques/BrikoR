use std::fs;
use std::error::Error;
use roxmltree::Document;

#[derive(Debug)]
struct DatabaseConfig {
    server: String,
    database_name: String,
    schema_name: String,
}

fn xml_config_read() -> Result<DatabaseConfig, Box<dyn Error>> {
    let xml = fs::read_to_string("config.xml")?;
    let doc = roxmltree::Document::parse(&xml)?;

    // 1. Find the <database> element safely
    let db = doc
        .root_element()
        .children()
        .find(|n| n.has_tag_name("database"))
        .ok_or("Missing <database> root element in config.xml")?;

    // 2. Helper closure or function to extract child text safely
    let get_text = |tag_name: &str| -> Result<String, Box<dyn Error>> {
        let child = db
            .children()
            .find(|n| n.has_tag_name(tag_name))
            .ok_or_else(|| format!("Missing <{}> tag inside <database>", tag_name))?;

        let text = child
            .text()
            .ok_or_else(|| format!("<{}> tag is empty", tag_name))?;

        Ok(text.to_string())
    };

    // 3. Extract fields cleanly with the ? operator
    let server = get_text("server")?;
    let database_name = get_text("database_name")?;
    let schema_name = get_text("schema_name")?;

    let config = DatabaseConfig {
        server,
        database_name,
        schema_name,
    };

    println!("Successfully loaded config: {:?}", config);
    Ok(config)
}




pub fn agent_read_all_db(file_path: &str) {
    // first read the .xml file-content into a string:
    // 1. Read the file contents from disk into a String
    let xml_data = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path, e);
            return;
        }
    };    

    let doc = match Document::parse(&xml_data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("XML parse error: {}", e);
            return;
        }
    };

    // Locate the <database> root element first
    if let Some(database_node) = doc.descendants().find(|n| n.has_tag_name("database")) {
        // Iterate through each child node named "server" under <database>
        for srv in database_node.children().filter(|n| n.has_tag_name("server")) {
            // Helper closure to safely extract child element text
            let get_child_text = |tag: &str| -> &str {
                srv.children()
                    .find(|c| c.has_tag_name(tag))
                    .and_then(|c| c.text())
                    .unwrap_or("")
            };

            let server_name = srv.attribute("name").unwrap_or("");
            let port = get_child_text("port");
            let db_name = get_child_text("db_name");
            let user = get_child_text("user");
            let password = get_child_text("password");
            let schema = get_child_text("schema");

            let conn_str = format!(
                "host={} port={} dbname={} user={} password={} options=-csearch_path={}",
                server_name, port, db_name, user, password, schema
            );

            println!("Loaded connection for server: {}", conn_str);
        }
    } else {
        eprintln!("Error: <database> block not found in XML.");
    }
}


fn agent_fetch_single_connection(xml_data: &str) {
let doc = match roxmltree::Document::parse(xml_data) {
    Ok(d) => d,
    Err(e) => {
        eprintln!("XML parse error: {}", e);
        return; // or return an Err/None depending on your function signature
    }
};
    

    // Find the server node matching both attribute and child text criteria
    let server_node = doc.descendants().find(|n| {
        n.has_tag_name("server")
            && n.attribute("name") == Some("Cantor")
            && n.children()
                .any(|c| c.has_tag_name("db-model") && c.text() == Some("PostgreSql"))
    });

    if let Some(srv) = server_node {
        // Helper closure to safely extract child element text
        let get_child_text = |tag: &str| -> &str {
            srv.children()
                .find(|c| c.has_tag_name(tag))
                .and_then(|c| c.text())
                .unwrap_or("")
        };

        let server_name = srv.attribute("name").unwrap_or("");
        let port = get_child_text("port");
        let db_name = get_child_text("db_name");
        let user = get_child_text("user");
        let password = get_child_text("password");
        let schema = get_child_text("schema");

        let conn_str = format!(
            "host={} port={} dbname={} user={} password={} options=-csearch_path={}",
            server_name, port, db_name, user, password, schema
        );

        println!("Loaded connection for target server: {}", conn_str);
    } else {
        eprintln!("Error: Specified server configuration not found in XML.");
    }
}// agent_fetch_single_connection
