use crate::{objects::Curve, validation::ValidationError};

use super::{Validate, ValidationConfig};

impl Validate for Curve {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
