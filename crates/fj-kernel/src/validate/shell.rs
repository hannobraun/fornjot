use std::{collections::HashMap, iter::repeat};

use fj_math::{Point, Scalar};

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
        ShellValidationError::validate_edges_coincident(self, config, errors);
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
        "Shell contains HalfEdges that are coinciendent but refer to different GlobalEdges\n
        Edge 1: {0:#?}
        Edge 2: {1:#?}
        "
    )]
    CoincidentEdgesNotIdentical(Handle<HalfEdge>, Handle<HalfEdge>),
    /// [`Shell`] contains half_edges that are identical, but do not coincide
    #[error(
        "Shell contains HalfEdges that are identical but do not coincide\n
        Edge 1: {0:#?}
        Edge 2: {1:#?}
        "
    )]
    IdenticalEdgesNotCoincident(Handle<HalfEdge>, Handle<HalfEdge>),
}

/// Sample two edges at various (currently 3) points in 3D along them.
///
/// Returns an [`Iterator`] of the distance at each sample.
fn distances(
    config: &ValidationConfig,
    (edge1, surface1): (Handle<HalfEdge>, SurfaceGeometry),
    (edge2, surface2): (Handle<HalfEdge>, SurfaceGeometry),
) -> impl Iterator<Item = Scalar> {
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

    // Three samples (start, middle, end), are enough to detect weather lines
    // and circles match. If we were to add more complicated curves, this might
    // need to change.
    let sample_count = 3;
    let step = 1.0 / (sample_count as f64 - 1.0);

    let mut distances = Vec::new();
    for i in 0..sample_count {
        let percent = i as f64 * step;
        let sample1 = sample(percent, (&edge1, surface1));
        let sample2 = sample(
            if flip { 1.0 - percent } else { percent },
            (&edge2, surface2),
        );
        distances.push(sample1.distance_to(&sample2))
    }
    distances.into_iter()
}

impl ShellValidationError {
    fn validate_edges_coincident(
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

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for edge in &faces {
            for other_edge in &faces {
                let id = edge.0.global_form().id();
                let other_id = other_edge.0.global_form().id();
                let identical = id == other_id;
                match identical {
                    true => {
                        // All points on identical curves should be within
                        // identical_max_distance, so we shouldn't have any
                        // greater than the max
                        if distances(config, edge.clone(), other_edge.clone())
                            .any(|d| d > config.identical_max_distance)
                        {
                            errors.push(
                                Self::IdenticalEdgesNotCoincident(
                                    edge.0.clone(),
                                    other_edge.0.clone(),
                                )
                                .into(),
                            )
                        }
                    }
                    false => {
                        // If all points on distinct curves are within
                        // distinct_min_distance, that's a problem.
                        if distances(config, edge.clone(), other_edge.clone())
                            .all(|d| d < config.distinct_min_distance)
                        {
                            errors.push(
                                Self::CoincidentEdgesNotIdentical(
                                    edge.0.clone(),
                                    other_edge.0.clone(),
                                )
                                .into(),
                            )
                        }
                    }
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
        if half_edge_to_faces.iter().any(|(_, c)| *c != 2) {
            errors.push(Self::NotWatertight.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_contains_err;
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

        assert_contains_err!(
            invalid,
            ValidationError::Shell(
                ShellValidationError::CoincidentEdgesNotIdentical(..)
            )
        );

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

        assert_contains_err!(
            invalid,
            ValidationError::Shell(ShellValidationError::NotWatertight)
        );

        Ok(())
    }
}
