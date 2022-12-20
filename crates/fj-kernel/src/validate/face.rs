use fj_math::Winding;

use crate::{
    objects::{Cycle, Face, Surface},
    storage::Handle,
};

use super::{Validate, ValidationConfig};

impl Validate for Face {
    type Error = FaceValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        FaceValidationError::check_surface_identity(self)?;
        FaceValidationError::check_interior_winding(self)?;
        Ok(())
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
    fn check_surface_identity(face: &Face) -> Result<(), Self> {
        let surface = face.surface();

        for interior in face.interiors() {
            if surface.id() != interior.surface().id() {
                return Err(Self::SurfaceMismatch {
                    surface: surface.clone(),
                    interior: interior.clone(),
                    face: face.clone(),
                });
            }
        }

        Ok(())
    }

    fn check_interior_winding(face: &Face) -> Result<(), Self> {
        let exterior_winding = face.exterior().winding();

        for interior in face.interiors() {
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                return Err(Self::InvalidInteriorWinding {
                    exterior_winding,
                    interior_winding,
                    face: face.clone(),
                });
            }
            assert_ne!(
                exterior_winding,
                interior.winding(),
                "Interior cycles must have opposite winding of exterior cycle"
            );
        }

        Ok(())
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
    fn face_surface_mismatch() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();

        let valid = {
            let mut face = PartialFace::default();
            face.exterior.write().surface = Partial::from(surface.clone());
            face.update_exterior_as_polygon([[0., 0.], [3., 0.], [0., 3.]]);
            face.add_interior_polygon(surface, [[1., 1.], [1., 2.], [2., 1.]]);

            face.build(&mut services.objects)
        };
        let invalid = {
            let mut cycle = PartialCycle {
                surface: Partial::from(services.objects.surfaces.xz_plane()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[1., 1.], [1., 2.], [2., 1.]]);
            let cycle = cycle
                .build(&mut services.objects)
                .insert(&mut services.objects);

            let interiors = [cycle];
            Face::new(valid.exterior().clone(), interiors, valid.color())
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn face_invalid_interior_winding() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();

        let valid = {
            let mut face = PartialFace::default();
            face.exterior.write().surface = Partial::from(surface.clone());
            face.update_exterior_as_polygon([[0., 0.], [3., 0.], [0., 3.]]);
            face.add_interior_polygon(surface, [[1., 1.], [1., 2.], [2., 1.]]);
            face.build(&mut services.objects)
        };
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services.objects))
                .collect::<Vec<_>>();

            Face::new(valid.exterior().clone(), interiors, valid.color())
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }
}
