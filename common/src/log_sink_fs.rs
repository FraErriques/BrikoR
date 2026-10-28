use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use chrono::{DateTime, Local};

/// Represents a tracing tag held in the stack.
#[derive(Clone, Debug)]
pub struct CurrentTag {
    pub tag_name: String,
    pub section_verbosity: i32,
}

impl Default for CurrentTag {
    fn default() -> Self {
        Self {
            tag_name: "unspecified".to_string(),
            section_verbosity: 0,
        }
    }
}

impl CurrentTag {
    pub fn new(tag_name: impl Into<String>, section_verbosity: i32) -> Self {
        Self {
            tag_name: tag_name.into(),
            section_verbosity,
        }
    }
}

/// Sink FileSys Stream log implementation in Rust.
pub struct SinkFs {
    file_path: String,
    tag_stack: Vec<CurrentTag>,
    indentment_level: usize,
    has_permissions_to_write: bool,
    semaphore: i32,
    verbosity: i32,
}

// Global mutex to ensure thread safety during critical log sections, mirroring C++ SinkFs_mtx
static SINK_FS_MTX: Mutex<()> = Mutex::new(());

impl SinkFs {
    /// Constructor: Initializes configuration checks, file paths, and permissions.
    pub fn new() -> Self {
        let _lock = SINK_FS_MTX.lock().unwrap();

        let mut instance = Self {
            file_path: String::new(),
            tag_stack: Vec::new(),
            indentment_level: 0,
            has_permissions_to_write: false,
            semaphore: 0,
            verbosity: 0,
        };

        let config_path = "./LogSinkFsConfig.txt";
        if !Path::new(config_path).exists() {
            // Create a HOWTO file if configuration is missing
            if let Ok(mut howto_file) = File::create("./LogConfig_HOWTO.txt") {
                let _ = howto_file.write_all(
                    b"Please create a file called LogSinkFsConfig.txt just in the directory where you find the current file. Then fill it as follows:\n\
                    key_logFullPathPrefix PARAGRAPH NAME\n\
                    key_semaphore PARAGRAPH NAME\n\
                    key_verbosity PARAGRAPH NAME\n"
                );
            }
            println!("Missing configuration file in AppDomain curDir; look for LogConfig_HOWTO.txt");
            return instance;
        }

        // Configuration parsing (simplified robust check mapping)
        // In a complete port, you would parse your configuration service here.
        // For demonstration, we set baseline operational defaults if config exists:
        instance.semaphore = 1; // Green
        instance.verbosity = 0; // Full verbosity
        instance.has_permissions_to_write = true;

        // Generate daily log file name with timestamping (using chrono crate)
        let now: DateTime<Local> = Local::now();
        let date_str = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        instance.file_path = format!("./LogStream_{}.log", date_str);

        // Try opening and writing header footer
        if let Ok(mut file) = instance.try_open_file() {
            let time_stamp = now.format("%H:%M:%S").to_string();
            let header_msg = format!("\r\n S=start, C=content, K=close -=separator \t{}\t\r\n\n", time_stamp);
            let _ = file.write_all(header_msg.as_bytes());
        } else {
            instance.has_permissions_to_write = false;
        }

        instance
    }

    pub fn get_current_log_name(&self) -> &str {
        &self.file_path
    }

    /// Helper to open file for appending or creation
    fn try_open_file(&self) -> std::io::Result<fs::File> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
    }

    fn write_indent(&self, file: &mut fs::File) -> std::io::Result<()> {
        for _ in 0..self.indentment_level {
            file.write_all(b"   ")?;
        }
        Ok(())
    }

    pub fn section_open(&mut self, cur_tag: &str, section_verbosity: i32) {
        if !self.has_permissions_to_write {
            return;
        }
        let _lock = SINK_FS_MTX.lock().unwrap();

        let tag = CurrentTag::new(cur_tag, section_verbosity);
        self.tag_stack.push(tag);

        if section_verbosity < self.verbosity {
            return;
        }

        if let Ok(mut file) = self.try_open_file() {
            let _ = file.write_all(b"-\nS");
            let _ = self.write_indent(&mut file);
            let _ = writeln!(file, "{}_______start", cur_tag);
            self.indentment_level += 1;
        }
    }

    pub fn section_trace(&mut self, what: &str, section_verbosity: i32) {
        if section_verbosity < self.verbosity || !self.has_permissions_to_write {
            return;
        }
        let _lock = SINK_FS_MTX.lock().unwrap();

        let current_tag = self.tag_stack.last().cloned().unwrap_or_default();
        let now: DateTime<Local> = Local::now();
        let trace_stamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        if let Ok(mut file) = self.try_open_file() {
            let _ = file.write_all(b"C  ");
            let _ = self.write_indent(&mut file);
            let _ = writeln!(file, "inside {}: {} {}", current_tag.tag_name, trace_stamp, what);
        }
    }

    pub fn section_close(&mut self) {
        if !self.has_permissions_to_write {
            return;
        }
        let _lock = SINK_FS_MTX.lock().unwrap();

        let current_tag = match self.tag_stack.pop() {
            Some(tag) => tag,
            None => return,
        };

        if current_tag.section_verbosity < self.verbosity {
            return;
        }

        if self.indentment_level > 0 {
            self.indentment_level -= 1;
        }

        if let Ok(mut file) = self.try_open_file() {
            let _ = file.write_all(b"K");
            let _ = self.write_indent(&mut file);
            let _ = writeln!(file, "closing_____{}_______end", current_tag.tag_name);
        }
    }
}

impl Drop for SinkFs {
    fn drop(&mut self) {
        let _lock = SINK_FS_MTX.lock().unwrap();
        // Destructor cleanup logic automatically handles dropping vector stack resources safely
    }
}