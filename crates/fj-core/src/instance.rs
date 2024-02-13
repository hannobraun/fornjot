//! Main entry point to the `fj-core` API
//!
//! See [`Instance`].

use crate::{layers::Layers, validate::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
pub struct Instance {
    /// Event-sourced background services
    pub layers: Layers,
}

impl Instance {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        let layers = Layers::new();
        Self { layers }
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let layers = Layers::with_validation_config(config);
        Self { layers }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}
