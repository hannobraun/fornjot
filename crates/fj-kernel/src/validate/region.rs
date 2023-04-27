use fj_math::Winding;

use crate::objects::Region;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Region {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        RegionValidationError::check_interior_winding(self, errors);
    }
}

/// [`Region`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum RegionValidationError {
    /// Interior of [`Region`] has invalid winding; must be opposite of exterior
    #[error(
        "Interior of `Region` has invalid winding; must be opposite of exterior\n\
        - Winding of exterior cycle: {exterior_winding:#?}\n\
        - Winding of interior cycle: {interior_winding:#?}\n\
        - `Region`: {region:#?}"
    )]
    InvalidInteriorWinding {
        /// The winding of the [`Region`]'s exterior cycle
        exterior_winding: Winding,

        /// The winding of the invalid interior cycle
        interior_winding: Winding,

        /// The region
        region: Region,
    },
}

impl RegionValidationError {
    fn check_interior_winding(
        region: &Region,
        errors: &mut Vec<ValidationError>,
    ) {
        if region.exterior().half_edges().count() == 0 {
            // Can't determine winding, if the cycle has no half-edges. Sounds
            // like a job for a different validation check.
            return;
        }

        let exterior_winding = region.exterior().winding();

        for interior in region.interiors() {
            if interior.half_edges().count() == 0 {
                // Can't determine winding, if the cycle has no half-edges.
                // Sounds like a job for a different validation check.
                continue;
            }
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        region: region.clone(),
                    }
                    .into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::reverse::Reverse,
        assert_contains_err,
        builder::{CycleBuilder, FaceBuilder},
        objects::Region,
        services::Services,
        validate::{RegionValidationError, Validate, ValidationError},
    };

    #[test]
    fn region_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = FaceBuilder::new(services.objects.surfaces.xy_plane())
            .with_exterior(CycleBuilder::polygon(
                [[0., 0.], [3., 0.], [0., 3.]],
                &mut services,
            ))
            .with_interior(CycleBuilder::polygon(
                [[1., 1.], [1., 2.], [2., 1.]],
                &mut services,
            ))
            .build(&mut services);
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services))
                .collect::<Vec<_>>();

            Region::new(valid.exterior().clone(), interiors, valid.color())
        };

        valid.region().validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Face(
                RegionValidationError::InvalidInteriorWinding { .. }
            )
        );

        Ok(())
    }
}
