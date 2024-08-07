//! Main entry point to the `fj-core` API
//!
//! See [`Core`].

use crate::{
    algorithms::approx::Tolerance, layers::Layers, validation::ValidationConfig,
};

/// An instance of the Fornjot core
///
/// This is the main entry point to `fj-core`'s API.
pub struct Core {
    /// The layers of data that make up the state of a core instance
    pub layers: Layers,

    /// Default tolerance used for intermediate geometry representation
    pub default_tolerance: Tolerance,
}

impl Core {
    /// Construct an instance of `Core`
    pub fn new() -> Self {
        Self::from_layers(Layers::default())
    }

    /// Construct an instance of `Core`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let layers = Layers::with_validation_config(config);
        Self::from_layers(layers)
    }

    fn from_layers(layers: Layers) -> Self {
        let default_tolerance = Tolerance::from_scalar(0.001)
            .expect("Tolerance provided is larger than zero");

        Self {
            layers,
            default_tolerance,
        }
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
