use fj_core::{
    algorithms::{
        approx::Tolerance, bounding_volume::BoundingVolume,
        triangulate::Triangulate,
    },
    validate::ValidationConfig,
};

use crate::{handle_model, Result};

/// An instance of Fornjot
///
/// This is the main entry point into the Fornjot API
#[derive(Default)]
pub struct Instance {
    /// The instance of the Fornjot core
    pub core: fj_core::Instance,
}

impl Instance {
    /// Construct an instance of `Instance`
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an instance of `Instance`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let core = fj_core::Instance::with_validation_config(config);
        Self { core }
    }

    /// Export or display a model, according to CLI arguments
    ///
    /// This function is intended to be called by applications that define a
    /// model and want to provide a standardized CLI interface for dealing with
    /// that model.
    ///
    /// This function is used by Fornjot's own testing infrastructure, but is
    /// useful beyond that, when using Fornjot directly to define a model.
    pub fn process_model<M>(&mut self, model: &M) -> Result
    where
        for<'r> (&'r M, Tolerance): Triangulate,
        M: BoundingVolume<3>,
    {
        handle_model(model, &mut self.core)
    }
}
