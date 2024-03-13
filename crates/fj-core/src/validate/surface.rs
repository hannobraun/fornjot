use crate::{geometry::Geometry, objects::Surface};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Surface {
    fn validate(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
        _: &Geometry,
    ) {
    }
}
