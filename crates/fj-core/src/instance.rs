//! Main entry point to the `fj-core` API
//!
//! See [`Instance`].

use crate::{services::Services, validate::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to the Fornjot API.
pub struct Instance {
    /// Event-sourced background services
    pub services: Services,
}

impl Instance {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        let services = Services::new();
        Self { services }
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let services = Services::with_validation_config(config);
        Self { services }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}
