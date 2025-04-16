//! Main entry point to the `fj-core` API
//!
//! See [`Core`].

use fj_interop::Tolerance;

use crate::{layers::Layers, validation::ValidationConfig};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
pub struct Core {
    /// The layers of data that make up the state of a core instance
    pub layers: Layers,
}

impl Core {
    /// Construct an instance of `Core`
    pub fn new() -> Self {
        let layers = Layers::default();
        Self { layers }
    }

    /// Construct an instance of `Core`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let layers = Layers::with_validation_config(config);
        Self { layers }
    }

    /// Access the tolerance value used for intermediate geometry representation
    pub fn tolerance(&self) -> Tolerance {
        self.layers.validation.config.tolerance
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
