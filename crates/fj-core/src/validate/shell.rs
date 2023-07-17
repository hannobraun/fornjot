use std::{collections::HashMap, iter::repeat};

use fj_math::{Point, Scalar};

use crate::{
    geometry::SurfaceGeometry,
    objects::{HalfEdge, Shell, Surface},
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
    /// [`Shell`] contains global_edges not referred to by two half-edges
    #[error("Shell is not watertight")]
    NotWatertight,

    /// [`Shell`] contains half-edges that are coincident, but refer to
    /// different global_edges
    #[error(
        "`Shell` contains `HalfEdge`s that are coincident but refer to \
        different `GlobalEdge`s\n\
        Edge 1: {0:#?}\n\
        Edge 2: {1:#?}"
    )]
    CoincidentEdgesNotIdentical(Handle<HalfEdge>, Handle<HalfEdge>),

    /// [`Shell`] contains half-edges that are identical, but do not coincide
    #[error(
        "Shell contains HalfEdges that are identical but do not coincide\n\
        Edge 1: {edge_a:#?}\n\
        Surface for edge 1: {surface_a:#?}\n\
        Edge 2: {edge_b:#?}\n\
        Surface for edge 2: {surface_b:#?}"
    )]
    IdenticalEdgesNotCoincident {
        /// The first edge
        edge_a: Handle<HalfEdge>,

        /// The surface that the first edge is on
        surface_a: Handle<Surface>,

        /// The second edge
        edge_b: Handle<HalfEdge>,

        /// The surface that the second edge is on
        surface_b: Handle<Surface>,
    },
}

/// Sample two edges at various (currently 3) points in 3D along them.
///
/// Returns an [`Iterator`] of the distance at each sample.
fn distances(
    config: &ValidationConfig,
    edge_a: Handle<HalfEdge>,
    surface_a: Handle<Surface>,
    (edge_b, surface_b): (Handle<HalfEdge>, Handle<Surface>),
) -> impl Iterator<Item = Scalar> {
    fn sample(
        percent: f64,
        (edge, surface): (&Handle<HalfEdge>, SurfaceGeometry),
    ) -> Point<3> {
        let [start, end] = edge.boundary().inner;
        let path_coords = start + (end - start) * percent;
        let surface_coords = edge.path().point_from_path_coords(path_coords);
        surface.point_from_surface_coords(surface_coords)
    }

    // Check whether start positions do not match. If they don't treat second edge as flipped
    let flip = sample(0.0, (&edge_a, surface_a.geometry()))
        .distance_to(&sample(0.0, (&edge_b, surface_b.geometry())))
        > config.identical_max_distance;

    // Three samples (start, middle, end), are enough to detect weather lines
    // and circles match. If we were to add more complicated curves, this might
    // need to change.
    let sample_count = 3;
    let step = 1.0 / (sample_count as f64 - 1.0);

    let mut distances = Vec::new();
    for i in 0..sample_count {
        let percent = i as f64 * step;
        let sample1 = sample(percent, (&edge_a, surface_a.geometry()));
        let sample2 = sample(
            if flip { 1.0 - percent } else { percent },
            (&edge_b, surface_b.geometry()),
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
        let edges_and_surfaces: Vec<_> = shell
            .faces()
            .into_iter()
            .flat_map(|face| {
                face.region()
                    .all_cycles()
                    .flat_map(|cycle| cycle.half_edges().cloned())
                    .zip(repeat(face.surface().clone()))
            })
            .collect();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (edge_a, surface_a) in &edges_and_surfaces {
            for (edge_b, surface_b) in &edges_and_surfaces {
                let id = edge_a.global_form().id();
                let other_id = edge_b.global_form().id();

                let identical = id == other_id;

                match identical {
                    true => {
                        // All points on identical curves should be within
                        // identical_max_distance, so we shouldn't have any
                        // greater than the max
                        if distances(
                            config,
                            edge_a.clone(),
                            surface_a.clone(),
                            (edge_b.clone(), surface_b.clone()),
                        )
                        .any(|d| d > config.identical_max_distance)
                        {
                            errors.push(
                                Self::IdenticalEdgesNotCoincident {
                                    edge_a: edge_a.clone(),
                                    surface_a: surface_a.clone(),
                                    edge_b: edge_b.clone(),
                                    surface_b: surface_b.clone(),
                                }
                                .into(),
                            )
                        }
                    }
                    false => {
                        // If all points on distinct curves are within
                        // distinct_min_distance, that's a problem.
                        if distances(
                            config,
                            edge_a.clone(),
                            surface_a.clone(),
                            (edge_b.clone(), surface_b.clone()),
                        )
                        .all(|d| d < config.distinct_min_distance)
                        {
                            errors.push(
                                Self::CoincidentEdgesNotIdentical(
                                    edge_a.clone(),
                                    edge_b.clone(),
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
        let mut half_edge_to_faces: HashMap<ObjectId, usize> = HashMap::new();

        for face in shell.faces() {
            for cycle in face.region().all_cycles() {
                for half_edge in cycle.half_edges() {
                    let id = half_edge.global_form().id();
                    let entry = half_edge_to_faces.entry(id);
                    *entry.or_insert(0) += 1;
                }
            }
        }

        // Each global edge should have exactly two half edges that are part of
        // the shell
        if half_edge_to_faces.iter().any(|(_, c)| *c != 2) {
            errors.push(Self::NotWatertight.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{GlobalEdge, Shell},
        operations::{
            BuildShell, Insert, UpdateCycle, UpdateFace, UpdateHalfEdge,
            UpdateRegion, UpdateShell,
        },
        services::Services,
        validate::{shell::ShellValidationError, Validate, ValidationError},
    };

    #[test]
    fn coincident_not_identical() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut services,
        );
        let invalid = valid.shell.replace_face(
            &valid.abc.face,
            valid
                .abc
                .face
                .update_region(|region| {
                    region
                        .update_exterior(|cycle| {
                            cycle
                                .update_nth_half_edge(0, |half_edge| {
                                    let global_form =
                                        GlobalEdge::new().insert(&mut services);
                                    half_edge
                                        .replace_global_form(global_form)
                                        .insert(&mut services)
                                })
                                .insert(&mut services)
                        })
                        .insert(&mut services)
                })
                .insert(&mut services),
        );

        valid.shell.validate_and_return_first_error()?;
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

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut services,
        );
        let invalid = valid.shell.remove_face(&valid.abc.face);

        valid.shell.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Shell(ShellValidationError::NotWatertight)
        );

        Ok(())
    }
}
