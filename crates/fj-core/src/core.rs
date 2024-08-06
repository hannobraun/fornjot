//! Main entry point to the `fj-core` API
//!
//! See [`Core`].

use crate::{layers::Layers, validation::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
pub struct Core {
    /// The layers of data that make up the state of a core instance
    pub layers: Layers,
}

impl Core {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        Self {
            layers: Layers::default(),
        }
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let layers = Layers::with_validation_config(config);
        Self { layers }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
