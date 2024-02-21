use crate::{
    objects::Curve,
    validation::{ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for Curve {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
