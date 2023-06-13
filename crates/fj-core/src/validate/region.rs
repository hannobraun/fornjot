use crate::objects::Region;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Region {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
