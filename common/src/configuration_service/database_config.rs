use std::fs;
use std::error::Error;

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