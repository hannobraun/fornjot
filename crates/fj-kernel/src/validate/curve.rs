use crate::objects::{Curve, GlobalCurve};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Curve {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}

impl Validate for GlobalCurve {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
