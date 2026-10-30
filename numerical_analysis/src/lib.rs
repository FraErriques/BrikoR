pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


// src/lib.rs
pub mod real_matrix;
pub mod monte_carlo;

// Re-export it so external crates can call Numerics::RealMatrix easily if desired, 
// or keep it under the module namespace.
pub use real_matrix::RealMatrix;
pub use monte_carlo::{
    BaseConverter, 
    ClassicalContinuousGenerator, 
    ClassicalDiscreteGenerator, 
    RandCpp11
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
