// src/monte_carlo/mod.rs

pub mod base_converter;
pub mod delta_omega;
pub mod modern_rand;
pub mod classical_continuous_generator;
pub mod classical_discrete_generator;

// Re-export key structures for convenient access
pub use base_converter::BaseConverter;
pub use delta_omega::DeltaOmega;
pub use modern_rand::RandCpp11;
pub use classical_continuous_generator::ClassicalContinuousGenerator;
pub use classical_discrete_generator::ClassicalDiscreteGenerator;