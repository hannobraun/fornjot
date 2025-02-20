use crate::{
    geometry::Geometry,
    topology::Shell,
    validation::{
        ValidationCheck,
        checks::{CoincidentHalfEdgesAreNotSiblings, HalfEdgeHasNoSibling},
    },
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            HalfEdgeHasNoSibling::check(self, geometry, config).map(Into::into),
        );
        errors.extend(
            CoincidentHalfEdgesAreNotSiblings::check(self, geometry, config)
                .map(Into::into),
        );
    }
}
