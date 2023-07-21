use std::collections::{BTreeMap, HashMap};

use fj_math::{Point, Scalar};

use crate::{
    geometry::SurfaceGeometry,
    objects::{HalfEdge, Shell, Surface},
    queries::{AllEdgesWithSurface, BoundingVerticesOfEdge},
    storage::{Handle, HandleWrapper, ObjectId},
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        ShellValidationError::validate_curve_coordinates(self, config, errors);
        ShellValidationError::validate_edges_coincident(self, config, errors);
        ShellValidationError::validate_watertight(self, config, errors);
        ShellValidationError::validate_same_orientation(self, errors);
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {
    /// [`Shell`] contains curves whose coordinate systems don't match
    #[error(
        "Curve coordinate system mismatch ({} errors): {:#?}",
        .0.len(),
        .0
    )]
    CurveCoordinateSystemMismatch(Vec<CurveCoordinateSystemMismatch>),

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

    /// [`Shell`] contains faces of mixed orientation (inwards and outwards)
    #[error("Shell has mixed face orientations")]
    MixedOrientations,
}

/// Sample two edges at various (currently 3) points in 3D along them.
///
/// Returns an [`Iterator`] of the distance at each sample.
fn distances(
    edge_a: Handle<HalfEdge>,
    surface_a: Handle<Surface>,
    edge_b: Handle<HalfEdge>,
    surface_b: Handle<Surface>,
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

    // Three samples (start, middle, end), are enough to detect weather lines
    // and circles match. If we were to add more complicated curves, this might
    // need to change.
    let sample_count = 3;
    let step = 1.0 / (sample_count as f64 - 1.0);

    let mut distances = Vec::new();
    for i in 0..sample_count {
        let percent = i as f64 * step;
        let sample1 = sample(percent, (&edge_a, surface_a.geometry()));
        let sample2 = sample(1.0 - percent, (&edge_b, surface_b.geometry()));
        distances.push(sample1.distance_to(&sample2))
    }
    distances.into_iter()
}

impl ShellValidationError {
    fn validate_curve_coordinates(
        shell: &Shell,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut edges_and_surfaces = Vec::new();
        shell.all_edges_with_surface(&mut edges_and_surfaces);

        for (edge_a, surface_a) in &edges_and_surfaces {
            for (edge_b, surface_b) in &edges_and_surfaces {
                // We only care about edges referring to the same curve.
                if edge_a.curve().id() != edge_b.curve().id() {
                    continue;
                }

                // No need to check an edge against itself.
                if edge_a.id() == edge_b.id() {
                    continue;
                }

                fn compare_curve_coords(
                    edge_a: &Handle<HalfEdge>,
                    surface_a: &Handle<Surface>,
                    edge_b: &Handle<HalfEdge>,
                    surface_b: &Handle<Surface>,
                    config: &ValidationConfig,
                    mismatches: &mut Vec<CurveCoordinateSystemMismatch>,
                ) {
                    // Let's check 4 points. Given that the most complex curves
                    // we have right now are circles, 3 would be enough to check
                    // for coincidence. But the first and last might be
                    // identical, so let's add an extra one.
                    let [a, d] = edge_a.boundary().inner;
                    let b = a + (d - a) * 1. / 3.;
                    let c = a + (d - a) * 2. / 3.;

                    for point_curve in [a, b, c, d] {
                        let a_surface =
                            edge_a.path().point_from_path_coords(point_curve);
                        let b_surface =
                            edge_b.path().point_from_path_coords(point_curve);

                        let a_global = surface_a
                            .geometry()
                            .point_from_surface_coords(a_surface);
                        let b_global = surface_b
                            .geometry()
                            .point_from_surface_coords(b_surface);

                        let distance = (a_global - b_global).magnitude();

                        if distance > config.identical_max_distance {
                            mismatches.push(CurveCoordinateSystemMismatch {
                                edge_a: edge_a.clone(),
                                edge_b: edge_b.clone(),
                                point_curve,
                                point_a: a_global,
                                point_b: b_global,
                                distance,
                            });
                        }
                    }
                }

                let mut mismatches = Vec::new();

                compare_curve_coords(
                    edge_a,
                    surface_a,
                    edge_b,
                    surface_b,
                    config,
                    &mut mismatches,
                );
                compare_curve_coords(
                    edge_b,
                    surface_b,
                    edge_a,
                    surface_a,
                    config,
                    &mut mismatches,
                );

                if !mismatches.is_empty() {
                    errors.push(
                        Self::CurveCoordinateSystemMismatch(mismatches).into(),
                    );
                }
            }
        }
    }

    fn validate_edges_coincident(
        shell: &Shell,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut edges_and_surfaces = Vec::new();
        shell.all_edges_with_surface(&mut edges_and_surfaces);

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (edge_a, surface_a) in &edges_and_surfaces {
            for (edge_b, surface_b) in &edges_and_surfaces {
                // No need to check an edge against itself.
                if edge_a.id() == edge_b.id() {
                    continue;
                }

                let identical_according_to_global_form =
                    edge_a.global_form().id() == edge_b.global_form().id();

                let identical_according_to_curve = {
                    let on_same_curve =
                        edge_a.curve().id() == edge_b.curve().id();

                    let have_same_boundary = {
                        let bounding_vertices_of = |edge| {
                            shell
                                .bounding_vertices_of_edge(edge)
                                .expect("Expected edge to be part of shell")
                                .normalize()
                        };

                        bounding_vertices_of(edge_a)
                            == bounding_vertices_of(edge_b)
                    };

                    on_same_curve && have_same_boundary
                };

                assert_eq!(
                    identical_according_to_curve,
                    identical_according_to_global_form,
                );

                match identical_according_to_curve {
                    true => {
                        // All points on identical curves should be within
                        // identical_max_distance, so we shouldn't have any
                        // greater than the max
                        if distances(
                            edge_a.clone(),
                            surface_a.clone(),
                            edge_b.clone(),
                            surface_b.clone(),
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
                            edge_a.clone(),
                            surface_a.clone(),
                            edge_b.clone(),
                            surface_b.clone(),
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
        let mut num_edges = BTreeMap::new();

        for face in shell.faces() {
            for cycle in face.region().all_cycles() {
                for half_edge in cycle.half_edges() {
                    let curve = HandleWrapper::from(half_edge.curve().clone());
                    let bounding_vertices = cycle
                        .bounding_vertices_of_edge(half_edge)
                        .expect(
                            "Cycle should provide bounds of its own half-edge",
                        )
                        .normalize();

                    let edge = (curve, bounding_vertices);

                    *num_edges.entry(edge).or_insert(0) += 1;
                }
            }
        }

        // Every edge should have exactly one matching edge that shares a curve
        // and boundary.
        if num_edges.into_values().any(|num| num != 2) {
            errors.push(Self::NotWatertight.into());
        }

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

    fn validate_same_orientation(
        shell: &Shell,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut orientations = shell
            .faces()
            .into_iter()
            .map(|f| f.region().exterior().winding());
        let first = orientations.next().unwrap();
        if !orientations.all(|elem| elem == first) {
            errors.push(Self::MixedOrientations.into())
        }
    }
}

#[derive(Clone, Debug)]
pub struct CurveCoordinateSystemMismatch {
    pub edge_a: Handle<HalfEdge>,
    pub edge_b: Handle<HalfEdge>,
    pub point_curve: Point<1>,
    pub point_a: Point<3>,
    pub point_b: Point<3>,
    pub distance: Scalar,
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{Curve, GlobalEdge, Shell},
        operations::{
            BuildShell, Insert, Reverse, UpdateCycle, UpdateFace,
            UpdateHalfEdge, UpdateRegion, UpdateShell,
        },
        services::Services,
        validate::{shell::ShellValidationError, Validate, ValidationError},
    };

    #[test]
    fn curve_coordinate_system_mismatch() -> anyhow::Result<()> {
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
                                    half_edge
                                        .replace_path(
                                            half_edge.path().reverse(),
                                        )
                                        .replace_boundary(
                                            half_edge.boundary().reverse(),
                                        )
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
                ShellValidationError::CurveCoordinateSystemMismatch(..)
            )
        );

        Ok(())
    }

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
                                    let curve =
                                        Curve::new().insert(&mut services);
                                    let global_form =
                                        GlobalEdge::new().insert(&mut services);

                                    half_edge
                                        .replace_curve(curve)
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
    #[test]
    fn shell_mixed_orientations() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut services,
        );
        let invalid = valid.shell.replace_face(
            &valid.abc.face,
            valid.abc.face.reverse(&mut services).insert(&mut services),
        );

        valid.shell.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Shell(ShellValidationError::MixedOrientations)
        );

        Ok(())
    }
}
