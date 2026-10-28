pub mod legacy;
pub mod modern;

// Re-export for convenience so users can call Common::configuration_service::ConfigurationService
pub use legacy::ConfigurationService;
pub use modern::ModernConfig;