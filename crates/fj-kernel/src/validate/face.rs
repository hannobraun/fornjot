use crate::objects::Face;

use super::{Validate2, ValidationConfig};

impl Validate2 for Face {
    type Error = FaceValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// [`Face`] validation error
#[derive(Debug, thiserror::Error)]
pub enum FaceValidationError {}
