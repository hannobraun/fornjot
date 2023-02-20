use fj_math::Winding;

use crate::{
    objects::{Cycle, Face, Surface},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_surface_identity(self, errors);
        FaceValidationError::check_interior_winding(self, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// [`Surface`] of an interior [`Cycle`] doesn't match [`Face`]'s `Surface`
    #[error(
        "`Surface` of an interior `Cycle` doesn't match `Face`'s `Surface`\n\
        - `Surface` of the `Face`: {surface:#?}\n\
        - Invalid interior `Cycle`: {interior:#?}\n\
        - `Face`: {face:#?}"
    )]
    SurfaceMismatch {
        /// The surface of the [`Face`]
        surface: Handle<Surface>,

        /// The invalid interior cycle of the [`Face`]
        interior: Handle<Cycle>,

        /// The face
        face: Face,
    },

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
    fn check_surface_identity(face: &Face, errors: &mut Vec<ValidationError>) {
        let surface = face.surface();

        for interior in face.interiors() {
            if surface.id() != interior.surface().id() {
                errors.push(
                    Box::new(Self::SurfaceMismatch {
                        surface: surface.clone(),
                        interior: interior.clone(),
                        face: face.clone(),
                    })
                    .into(),
                );
            }
        }
    }

    fn check_interior_winding(face: &Face, errors: &mut Vec<ValidationError>) {
        let exterior_winding = face.exterior().winding();

        for interior in face.interiors() {
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Box::new(Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        face: face.clone(),
                    })
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
        builder::{CycleBuilder, FaceBuilder},
        insert::Insert,
        objects::Face,
        partial::{Partial, PartialCycle, PartialFace, PartialObject},
        services::Services,
        validate::Validate,
    };

    #[test]
    fn face_surface_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut face = PartialFace {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };
            face.exterior.write().surface = Partial::from(surface);
            face.exterior.write().update_as_polygon_from_points([
                [0., 0.],
                [3., 0.],
                [0., 3.],
            ]);
            face.add_interior().write().update_as_polygon_from_points([
                [1., 1.],
                [1., 2.],
                [2., 1.],
            ]);

            face.build(&mut services.objects)
        };
        let invalid = {
            let surface = services.objects.surfaces.xz_plane();

            let mut cycle = PartialCycle {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[1., 1.], [1., 2.], [2., 1.]]);
            cycle.infer_vertex_positions_if_necessary(&surface.geometry());
            let cycle = cycle
                .build(&mut services.objects)
                .insert(&mut services.objects);

            let interiors = [cycle];
            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn face_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut face = PartialFace {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };
            face.exterior.write().surface = Partial::from(surface);
            face.exterior.write().update_as_polygon_from_points([
                [0., 0.],
                [3., 0.],
                [0., 3.],
            ]);
            face.add_interior().write().update_as_polygon_from_points([
                [1., 1.],
                [1., 2.],
                [2., 1.],
            ]);
            face.build(&mut services.objects)
        };
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services.objects))
                .collect::<Vec<_>>();

            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
