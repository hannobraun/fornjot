use crate::{
    geometry::Geometry,
    topology::Face,
    validation::{ValidationCheck, ValidationConfig},
};

/// [`Face`] has no boundary
///
/// A face must have a boundary, meaning its exterior cycle must not be empty.
/// Checking *that* the exterior cycle is not empty is enough, as
/// [`AdjacentHalfEdgesNotConnected`] makes sure that any cycle that is not
/// empty, is closed.
///
/// [`AdjacentHalfEdgesNotConnected`]: super::AdjacentHalfEdgesNotConnected
#[derive(Clone, Debug, thiserror::Error)]
#[error("`Face` has no boundary")]
pub struct FaceHasNoBoundary {}

impl ValidationCheck<Face> for FaceHasNoBoundary {
    fn check<'r>(
        object: &'r Face,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let error = if object.region().exterior().half_edges().is_empty() {
            Some(FaceHasNoBoundary {})
        } else {
            None
        };

        error.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Core,
        operations::{
            build::{BuildCycle, BuildFace},
            update::{UpdateFace, UpdateRegion},
        },
        topology::{Cycle, Face},
        validation::{ValidationCheck, checks::FaceHasNoBoundary},
    };

    #[test]
    fn face_has_no_boundary() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Face::circle(
            core.layers.topology.surfaces.xy_plane(),
            [0., 0.],
            1.,
            &mut core,
        );
        FaceHasNoBoundary::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = valid.update_region(
            |region, core| region.update_exterior(|_, _| Cycle::empty(), core),
            &mut core,
        );
        FaceHasNoBoundary::check_and_expect_one_error(
            &invalid,
            &core.layers.geometry,
        );

        Ok(())
    }
}
