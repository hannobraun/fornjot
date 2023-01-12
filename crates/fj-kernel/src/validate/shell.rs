use crate::objects::Shell;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
