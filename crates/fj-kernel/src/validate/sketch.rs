use crate::objects::Sketch;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Sketch {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
