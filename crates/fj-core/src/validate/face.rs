use fj_math::Winding;

use crate::objects::Face;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_interior_winding(self, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// Interior of [`Face`] has invalid winding; must be opposite of exterior
    #[error(
        "Interior of `Face` has invalid winding; must be opposite of exterior\n\
        - Winding of exterior cycle: {exterior_winding:#?}\n\
        - Winding of interior cycle: {interior_winding:#?}\n\
        - `Face`: {face:#?}"
    )]
    InvalidInteriorWinding {
        /// The winding of the [`Face`]'s exterior cycle
        exterior_winding: Winding,

        /// The winding of the invalid interior cycle
        interior_winding: Winding,

        /// The face
        face: Face,
    },
}

impl FaceValidationError {
    fn check_interior_winding(face: &Face, errors: &mut Vec<ValidationError>) {
        if face.region().exterior().half_edges().is_empty() {
            // Can't determine winding, if the cycle has no edges. Sounds like a
            // job for a different validation check.
            return;
        }

        let exterior_winding = face.region().exterior().winding();

        for interior in face.region().interiors() {
            if interior.half_edges().is_empty() {
                // Can't determine winding, if the cycle has no edges. Sounds
                // like a job for a different validation check.
                continue;
            }
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        face: face.clone(),
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
        assert_contains_err,
        objects::{Cycle, Face, Region},
        operations::{
            BuildCycle, BuildFace, Insert, Reverse, UpdateFace, UpdateRegion,
        },
        services::Services,
        validate::{FaceValidationError, Validate, ValidationError},
    };

    #[test]
    fn face_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid =
            Face::unbound(services.objects.surfaces.xy_plane(), &mut services)
                .update_region(|region| {
                    region
                        .update_exterior(|_| {
                            Cycle::polygon(
                                [[0., 0.], [3., 0.], [0., 3.]],
                                &mut services,
                            )
                            .insert(&mut services)
                        })
                        .add_interiors([Cycle::polygon(
                            [[1., 1.], [1., 2.], [2., 1.]],
                            &mut services,
                        )
                        .insert(&mut services)])
                        .insert(&mut services)
                });
        let invalid = {
            let interiors = valid
                .region()
                .interiors()
                .iter()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services).insert(&mut services))
                .collect::<Vec<_>>();

            let region = Region::new(
                valid.region().exterior().clone(),
                interiors,
                valid.region().color(),
            )
            .insert(&mut services);

            Face::new(valid.surface().clone(), region)
        };

        valid.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Face(
                FaceValidationError::InvalidInteriorWinding { .. }
            )
        );

        services.only_validate(valid);

        Ok(())
    }
}
