use crate::objects::Cycle;

use super::{Validate2, ValidationConfig};

impl Validate2 for Cycle {
    type Error = CycleValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// [`Cycle`] validation error
#[derive(Debug, thiserror::Error)]
pub enum CycleValidationError {}
