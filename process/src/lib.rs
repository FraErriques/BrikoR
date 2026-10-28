pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// process/src/lib.rs
pub mod log_wrappers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
