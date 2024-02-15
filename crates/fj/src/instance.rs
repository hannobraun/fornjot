use fj_core::validate::ValidationConfig;

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
}
