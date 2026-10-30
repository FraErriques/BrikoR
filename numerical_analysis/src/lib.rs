pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


// src/lib.rs

pub mod monte_carlo;
pub mod real;
pub mod complex;
pub mod real_matrix;
pub mod complex_matrix;
pub mod real_integration;
pub mod integrate;
pub mod complex_integration;
// Prime Number Theory & Riemann-Zeta subfolder
pub mod primes;

// Re-export it so external crates can call Numerics::RealMatrix easily if desired, 
// or keep it under the module namespace.
pub use real::*;
pub use complex::Complex;
pub use monte_carlo::{
    BaseConverter, 
    ClassicalContinuousGenerator, 
    ClassicalDiscreteGenerator, 
    RandCpp11
};
pub use real_matrix::RealMatrix;
pub use complex_matrix::ComplexMatrix;
pub use real_integration::*;
pub use integrate::Integrate;
pub use complex_integration::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
