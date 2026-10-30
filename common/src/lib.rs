pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// common/src/lib.rs
pub mod configuration_service;
pub mod connection_provider;
pub mod log_sink_fs;
pub mod log_sink_db;
pub mod string_builder;
pub mod common_data;
pub use common_data::{Crash, Couple, Domain, PI};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
