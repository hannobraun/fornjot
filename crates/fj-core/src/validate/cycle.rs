use crate::{
    objects::Cycle,
    validation::{
        checks::AdjacentHalfEdgesNotConnected, ValidationCheck,
        ValidationConfig, ValidationError,
    },
};

use super::Validate;

impl Validate for Cycle {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        errors.extend(
            AdjacentHalfEdgesNotConnected::check(self, config).map(Into::into),
        );
    }
}
