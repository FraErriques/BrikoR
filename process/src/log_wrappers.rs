use common::log_sink_fs::SinkFs;
use std::sync::{Mutex, OnceLock};

/// Thread-safe global singleton instance of SinkFs, mirroring TSingleton<SinkFs>
fn global_logger() -> &'static Mutex<SinkFs> {
    static LOGGER: OnceLock<Mutex<SinkFs>> = OnceLock::new();
    LOGGER.get_or_init(|| Mutex::new(SinkFs::new()))
}

/// Requiring an instance without tracing produces the log file on disk.
pub fn just_construct_test(_name: &str, _section_verbosity: i32) {
    let _ = global_logger();
}

/// Placeholder for manual cleanup if needed (Rust handles drops automatically)
pub fn end_of_application_mandatory_() {
    // In Rust, resources drop naturally when the program exits or locks clear.
}

pub fn section_open(name: &str, section_verbosity: i32) {
    if let Ok(mut logger) = global_logger().lock() {
        logger.section_open(name, section_verbosity);
    }
}

pub fn section_content(content: &str, section_verbosity: i32) {
    if let Ok(mut logger) = global_logger().lock() {
        logger.section_trace(content, section_verbosity);
    }
}

pub fn section_close() {
    if let Ok(mut logger) = global_logger().lock() {
        logger.section_close();
    }
}

/// Multi-message wrapper for an array of strings
pub fn section_open_multi_message(message_array: &[String], section_verbosity: i32) {
    let combined = message_array.join(" ");
    section_open(&combined, section_verbosity);
}

pub fn section_content_multi_message(message_array: &[String], section_verbosity: i32) {
    let combined = message_array.join(" ");
    section_content(&combined, section_verbosity);
}

/// Generic variable-name/value logger. 
/// This single Rust function replaces all your overloaded unsigned long long, long long, double, and bool functions!
pub fn section_content_variable_name_value<T: std::fmt::Display>(
    variable_name: &str,
    variable_value: T,
    section_verbosity: i32,
) {
    let message = format!("{}  {}", variable_name, variable_value);
    section_content(&message, section_verbosity);
}

/// Macro to replace C-style variadic `SectionContent_fromMultipleStrings` safely and cleanly.
/// Usage: section_content_string_pack!(verbosity, "Item 1", item_var, 42);
#[macro_export]
macro_rules! section_content_string_pack {
    ($verbosity:expr, $($arg:expr),+ $(,)?) => {
        {
            let combined = format!($("{0}  ",)+ $($arg),+);
            // Trim trailing separator if needed, or pass directly
            $crate::log_wrappers::section_content(&combined, $verbosity);
        }
    };
}