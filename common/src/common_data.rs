/// Equivalent to common_data.h

pub type Domain = i16; // type for throwing exceptions / domain definitions

pub const PI: f64 = std::f64::consts::PI;

/// A type for Exception-Handling mirroring the C++ Crash struct
#[derive(Debug, Clone)]
pub struct Crash {
    pub message: String,
}

impl Crash {
    pub fn new(message: &str) -> Self {
        eprintln!("\n\t{}\n\t", message);
        Crash {
            message: message.to_string(),
        }
    }

    pub fn default_fault() -> Self {
        let message = "Memory Fault".to_string();
        eprintln!("\n\t{}\n\n", message);
        Crash { message }
    }
}

impl std::fmt::Display for Crash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Crash {}

/// Generic vector element of R2, used by interpolation methods
#[derive(Debug, Clone, Copy)]
pub struct Couple {
    pub argument: f64, // abscissa
    pub image: f64,    // ordinate
}