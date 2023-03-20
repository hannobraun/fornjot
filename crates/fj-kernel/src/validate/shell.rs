use std::{collections::HashMap, iter::repeat};

use fj_math::Point;

use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{HalfEdge, Shell},
    storage::{Handle, ObjectId},
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        ShellValidationError::validate_coincident(self, config, errors);
        ShellValidationError::validate_watertight(self, config, errors);
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {
    /// [`Shell`] contains global_edges not referred to by two half_edges
    #[error("Shell is not watertight")]
    NotWatertight,
    /// [`Shell`] contains half_edges that are coincident, but refer to different global_edges
    #[error(
        "Shell contains HalfEdges which are coinciendent but refer to different GlobalEdges\n
        Edge 1: {0:#?}
        Edge 2: {1:#?}
        "
    )]
    CoincidentEdgesNotIdentical(HalfEdge, HalfEdge),
}

/// Check whether to [`HalfEdge`]s are coincident. Along with the edges you
/// provide the surface that they are on, so that we can check their positions
/// in 3D space.
/// We check whether they are coincident by comparing a configurable amount
/// of points, and seeing whether they are further than the maximum distance for
/// identical objects, see [`ValidationConfig`]
fn are_coincident(
    config: &ValidationConfig,
    (edge1, surface1): (Handle<HalfEdge>, SurfaceGeometry),
    (edge2, surface2): (Handle<HalfEdge>, SurfaceGeometry),
) -> bool {
    fn sample(
        percent: f64,
        (edge, surface): (&Handle<HalfEdge>, SurfaceGeometry),
    ) -> Point<3> {
        let boundary = edge.boundary();
        let path_coords = boundary[0] + (boundary[1] - boundary[0]) * percent;
        let surface_coords = edge.curve().point_from_path_coords(path_coords);
        surface.point_from_surface_coords(surface_coords)
    }

    // Check whether start positions do not match. If they don't treat second edge as flipped
    let flip = sample(0.0, (&edge1, surface1))
        .distance_to(&sample(0.0, (&edge2, surface2)))
        > config.identical_max_distance;

    for i in 0..config.sample_count {
        let percent = i as f64 * (1.0 / config.sample_count as f64);
        let sample1 = sample(percent, (&edge1, surface1));
        let sample2 = sample(
            if flip { 1.0 - percent } else { percent },
            (&edge2, surface2),
        );
        if sample1.distance_to(&sample2) > config.identical_max_distance {
            // If corresponding points are further than the max distance than these HalfEdges are not coincident
            return false;
        }
    }

    true
}

impl ShellValidationError {
    fn validate_coincident(
        shell: &Shell,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let faces: Vec<(Handle<HalfEdge>, SurfaceGeometry)> = shell
            .faces()
            .into_iter()
            .flat_map(|face| {
                face.all_cycles()
                    .flat_map(|cycle| cycle.half_edges().cloned())
                    .zip(repeat(face.surface().geometry()))
            })
            .collect();

        // This is O(N) which isn't great, but we can't use a HashMap since we need to deal with float inaccuracies.
        #[allow(unreachable_code)]
        for edge in &faces {
            for other_edge in &faces {
                let identical = edge.0.global_form().id()
                    == other_edge.0.global_form().id();
                if are_coincident(config, edge.clone(), other_edge.clone())
                    && !identical
                {
                    errors.push(
                        Self::CoincidentEdgesNotIdentical(
                            edge.0.clone_object(),
                            other_edge.0.clone_object(),
                        )
                        .into(),
                    )
                }
            }
        }
    }

    fn validate_watertight(
        shell: &Shell,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let faces = shell.faces();
        let mut half_edge_to_faces: HashMap<ObjectId, usize> = HashMap::new();
        for face in faces {
            for cycle in face.all_cycles() {
                for half_edge in cycle.half_edges() {
                    let id = half_edge.global_form().id();
                    let entry = half_edge_to_faces.entry(id);
                    *entry.or_insert(0) += 1;
                }
            }
        }

        // Each global edge should have exactly two half edges that are part of the shell
        if half_edge_to_faces.iter().find(|(_, c)| **c != 2).is_some() {
            errors.push(Self::NotWatertight.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insert::Insert;

    use crate::validate::shell::ShellValidationError;
    use crate::{
        builder::{CycleBuilder, FaceBuilder},
        objects::Shell,
        services::Services,
        validate::{Validate, ValidationError},
    };

    #[test]
    fn coincident_not_identical() -> anyhow::Result<()> {
        let mut services = Services::new();
        let invalid = {
            // Shell with single face is not watertight
            let face1 = FaceBuilder::new(services.objects.surfaces.xy_plane())
                .with_exterior(CycleBuilder::polygon([
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                    [1., 0.],
                ]))
                .build(&mut services.objects)
                .insert(&mut services.objects);

            let face2 = FaceBuilder::new(services.objects.surfaces.xz_plane())
                .with_exterior(CycleBuilder::polygon([
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                    [1., 0.],
                ]))
                .build(&mut services.objects)
                .insert(&mut services.objects);

            Shell::new([face1, face2])
        };

        assert!({
            let mut errors = Vec::new();
            invalid.validate(&mut errors);
            errors
                .iter()
                .find(|e| {
                    matches!(
                        e,
                        ValidationError::Shell(
                            ShellValidationError::CoincidentEdgesNotIdentical(
                                ..
                            )
                        )
                    )
                })
                .is_some()
        });

        Ok(())
    }
    #[test]
    fn shell_not_watertight() -> anyhow::Result<()> {
        let mut services = Services::new();

        let invalid = {
            // Shell with single face is not watertight
            let face = FaceBuilder::new(services.objects.surfaces.xy_plane())
                .with_exterior(CycleBuilder::polygon([
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                    [1., 0.],
                ]))
                .build(&mut services.objects)
                .insert(&mut services.objects);
            Shell::new([face])
        };

        assert!({
            let mut errors = Vec::new();
            invalid.validate(&mut errors);
            errors
                .iter()
                .find(|e| {
                    matches!(
                        e,
                        ValidationError::Shell(
                            ShellValidationError::NotWatertight
                        )
                    )
                })
                .is_some()
        });

        Ok(())
    }
}
