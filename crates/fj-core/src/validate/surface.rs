use crate::objects::Surface;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Surface {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
