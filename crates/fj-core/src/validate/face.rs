use fj_math::Winding;

use crate::{
    geometry::Geometry,
    objects::Face,
    validation::{
        checks::FaceHasNoBoundary, ValidationCheck, ValidationConfig,
        ValidationError,
    },
};

use super::Validate;

impl Validate for Face {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            FaceHasNoBoundary::check(self, geometry, config).map(Into::into),
        );
        FaceValidationError::check_interior_winding(self, geometry, errors);
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
    fn check_interior_winding(
        face: &Face,
        geometry: &Geometry,
        errors: &mut Vec<ValidationError>,
    ) {
        if face.region().exterior().half_edges().is_empty() {
            // Can't determine winding, if the cycle has no edges. Sounds like a
            // job for a different validation check.
            return;
        }

        let exterior_winding = face.region().exterior().winding(geometry);

        for interior in face.region().interiors() {
            if interior.half_edges().is_empty() {
                // Can't determine winding, if the cycle has no edges. Sounds
                // like a job for a different validation check.
                continue;
            }
            let interior_winding = interior.winding(geometry);

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
            build::{BuildCycle, BuildFace},
            derive::DeriveFrom,
            insert::Insert,
            reverse::Reverse,
            update::{UpdateFace, UpdateRegion},
        },
        validate::{FaceValidationError, Validate},
        validation::ValidationError,
        Core,
    };

    #[test]
    fn interior_winding() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid =
            Face::unbound(core.layers.objects.surfaces.xy_plane(), &mut core)
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
                                )],
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
                    cycle
                        .reverse(&mut core)
                        .insert(&mut core)
                        .derive_from(&cycle, &mut core)
                })
                .collect::<Vec<_>>();

            let region =
                Region::new(valid.region().exterior().clone(), interiors)
                    .insert(&mut core);

            Face::new(valid.surface().clone(), region)
        };

        valid.validate_and_return_first_error(&core.layers.geometry)?;
        assert_contains_err!(
            core,
            invalid,
            ValidationError::Face(
                FaceValidationError::InvalidInteriorWinding { .. }
            )
        );

        Ok(())
    }
}
