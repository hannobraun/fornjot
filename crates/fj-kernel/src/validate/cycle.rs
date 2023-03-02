use crate::objects::Cycle;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Cycle {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
