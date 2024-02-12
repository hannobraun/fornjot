use fj_math::Winding;

use crate::objects::Face;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_boundary(self, errors);
        FaceValidationError::check_interior_winding(self, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// The [`Face`] has no exterior cycle
    #[error("The `Face` has no exterior cycle")]
    MissingBoundary,

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
    fn check_boundary(face: &Face, errors: &mut Vec<ValidationError>) {
        if face.region().exterior().half_edges().is_empty() {
            errors.push(ValidationError::from(Self::MissingBoundary));
        }

        // Checking *that* a boundary exists is enough. There are validation
        // checks for `Cycle` to make sure that the cycle is closed properly.
    }

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
        objects::{Cycle, Face, HalfEdge, Region},
        operations::{
            build::{BuildCycle, BuildFace, BuildHalfEdge},
            insert::Insert,
            reverse::Reverse,
            update::{UpdateCycle, UpdateFace, UpdateRegion},
        },
        validate::{FaceValidationError, Validate, ValidationError},
        Instance,
    };

    #[test]
    fn boundary() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let invalid =
            Face::unbound(core.services.objects.surfaces.xy_plane(), &mut core);
        let valid = invalid.update_region(
            |region, core| {
                region.update_exterior(
                    |cycle, core| {
                        cycle.add_half_edges(
                            [HalfEdge::circle([0., 0.], 1., core)],
                            core,
                        )
                    },
                    core,
                )
            },
            &mut core,
        );

        valid.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Face(FaceValidationError::MissingBoundary)
        );

        Ok(())
    }

    #[test]
    fn interior_winding() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let valid =
            Face::unbound(core.services.objects.surfaces.xy_plane(), &mut core)
                .update_region(
                    |region, core| {
                        region
                            .update_exterior(
                                |_, core| {
                                    Cycle::polygon(
                                        [[0., 0.], [3., 0.], [0., 3.]],
                                        core,
                                    )
                                },
                                core,
                            )
                            .add_interiors(
                                [Cycle::polygon(
                                    [[1., 1.], [1., 2.], [2., 1.]],
                                    core,
                                )
                                .insert(&mut core.services)],
                                core,
                            )
                    },
                    &mut core,
                );
        let invalid = {
            let interiors = valid
                .region()
                .interiors()
                .iter()
                .cloned()
                .map(|cycle| {
                    cycle.reverse(&mut core).insert(&mut core.services)
                })
                .collect::<Vec<_>>();

            let region = Region::new(
                valid.region().exterior().clone(),
                interiors,
                valid.region().color(),
            )
            .insert(&mut core.services);

            Face::new(valid.surface().clone(), region)
        };

        valid.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Face(
                FaceValidationError::InvalidInteriorWinding { .. }
            )
        );

        Ok(())
    }
}
