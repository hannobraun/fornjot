//! Main entry point to the `fj-core` API
//!
//! See [`Instance`].

use crate::{layers::Layers, validate::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
pub struct Instance {
    /// Event-sourced background services
    pub services: Layers,
}

impl Instance {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        let services = Layers::new();
        Self { services }
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let services = Layers::with_validation_config(config);
        Self { services }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}
