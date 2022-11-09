use crate::{
    objects::{Cycle, Face, Surface},
    storage::Handle,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for Face {
    type Error = FaceValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        FaceValidationError::check_surface_identity(self)?;
        Ok(())
    }
}

/// [`Face`] validation error
#[derive(Debug, thiserror::Error)]
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
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::CycleBuilder,
        objects::{Cycle, Face, Objects},
        partial::HasPartial,
        validate::Validate2,
    };

    #[test]
    fn face_surface_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = Face::builder(&objects)
            .with_surface(objects.surfaces.xy_plane())
            .with_exterior_polygon_from_points([[0., 0.], [3., 0.], [0., 3.]])
            .with_interior_polygon_from_points([[1., 1.], [1., 2.], [2., 1.]])
            .build();
        let invalid = {
            let interiors = [Cycle::partial()
                .with_poly_chain_from_points(
                    objects.surfaces.xz_plane(),
                    [[1., 1.], [1., 2.], [2., 1.]],
                )
                .close_with_line_segment()
                .build(&objects)?];

            Face::new(valid.exterior().clone(), interiors, valid.color())
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }
}
