use crate::objects::Curve;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Curve {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
