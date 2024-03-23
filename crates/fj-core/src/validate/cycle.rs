use crate::{
    geometry::Geometry,
    topology::Cycle,
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
        geometry: &Geometry,
    ) {
        errors.extend(
            AdjacentHalfEdgesNotConnected::check(self, geometry, config)
                .map(Into::into),
        );
    }
}
