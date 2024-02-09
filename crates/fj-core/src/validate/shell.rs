use std::{collections::BTreeMap, fmt};

use fj_math::{Point, Scalar};

use crate::{
    geometry::{CurveBoundary, SurfaceGeometry},
    objects::{Curve, HalfEdge, Shell, Surface, Vertex},
    queries::{
        AllHalfEdgesWithSurface, BoundingVerticesOfHalfEdge, SiblingOfHalfEdge,
    },
    storage::{Handle, HandleWrapper},
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        ShellValidationError::check_curve_coordinates(self, config, errors);
        ShellValidationError::check_half_edge_pairs(self, errors);
        ShellValidationError::check_half_edge_coincidence(self, config, errors);
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

    /// [`Shell`] contains a half-edge that is not part of a pair
    #[error("Half-edge has no sibling: {half_edge:#?}")]
    HalfEdgeHasNoSibling {
        /// The half-edge that has no sibling
        half_edge: Handle<HalfEdge>,
    },

    /// [`Shell`] contains half-edges that are coincident, but aren't siblings
    #[error(
        "`Shell` contains `HalfEdge`s that are coincident but are not \
        siblings\n\
        {boundaries}\
        {curves}\
        {vertices}\
        Half-edge 1: {half_edge_a:#?}\n\
        Half-edge 2: {half_edge_b:#?}"
    )]
    CoincidentHalfEdgesAreNotSiblings {
        /// The boundaries of the half-edges
        boundaries: Box<CoincidentHalfEdgeBoundaries>,

        /// The curves of the half-edges
        curves: Box<CoincidentHalfEdgeCurves>,

        /// The vertices of the half-edges
        vertices: Box<CoincidentHalfEdgeVertices>,

        /// The first half-edge
        half_edge_a: Handle<HalfEdge>,

        /// The second half-edge
        half_edge_b: Handle<HalfEdge>,
    },
}

impl ShellValidationError {
    /// Check that local curve definitions that refer to the same curve match
    fn check_curve_coordinates(
        shell: &Shell,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut edges_and_surfaces = Vec::new();
        shell.all_half_edges_with_surface(&mut edges_and_surfaces);

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
                                half_edge_a: edge_a.clone(),
                                half_edge_b: edge_b.clone(),
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

    /// Check that each half-edge is part of a pair
    fn check_half_edge_pairs(shell: &Shell, errors: &mut Vec<ValidationError>) {
        let mut unmatched_half_edges = BTreeMap::new();

        for face in shell.faces() {
            for cycle in face.region().all_cycles() {
                for half_edge in cycle.half_edges() {
                    let curve = HandleWrapper::from(half_edge.curve().clone());
                    let boundary = half_edge.boundary();
                    let vertices =
                        cycle.bounding_vertices_of_half_edge(half_edge).expect(
                            "`half_edge` came from `cycle`, must exist there",
                        );

                    let key = (curve.clone(), boundary, vertices.clone());
                    let key_reversed =
                        (curve, boundary.reverse(), vertices.reverse());

                    match unmatched_half_edges.remove(&key_reversed) {
                        Some(sibling) => {
                            // This must be the sibling of the half-edge we're
                            // currently looking at. Let's make sure the logic
                            // we use here to determine that matches the
                            // "official" definition.
                            assert!(shell.are_siblings(half_edge, sibling));
                        }
                        None => {
                            // If this half-edge has a sibling, we haven't seen
                            // it yet. Let's store this half-edge then, in case
                            // we come across the sibling later.
                            unmatched_half_edges.insert(key, half_edge);
                        }
                    }
                }
            }
        }

        for half_edge in unmatched_half_edges.into_values().cloned() {
            errors.push(Self::HalfEdgeHasNoSibling { half_edge }.into());
        }
    }

    /// Check that non-sibling half-edges are not coincident
    fn check_half_edge_coincidence(
        shell: &Shell,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut edges_and_surfaces = Vec::new();
        shell.all_half_edges_with_surface(&mut edges_and_surfaces);

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (half_edge_a, surface_a) in &edges_and_surfaces {
            for (half_edge_b, surface_b) in &edges_and_surfaces {
                // No need to check a half-edge against itself.
                if half_edge_a.id() == half_edge_b.id() {
                    continue;
                }

                if shell.are_siblings(half_edge_a, half_edge_b) {
                    // If the half-edges are siblings, they are allowed to be
                    // coincident. Must be, in fact. There's another validation
                    // check that takes care of that.
                    continue;
                }

                // If all points on distinct curves are within
                // `distinct_min_distance`, that's a problem.
                if distances(
                    half_edge_a.clone(),
                    surface_a.clone(),
                    half_edge_b.clone(),
                    surface_b.clone(),
                )
                .all(|d| d < config.distinct_min_distance)
                {
                    let boundaries = Box::new(CoincidentHalfEdgeBoundaries {
                        boundaries: [half_edge_a, half_edge_b]
                            .map(|half_edge| half_edge.boundary()),
                    });
                    let curves = Box::new(CoincidentHalfEdgeCurves {
                        curves: [half_edge_a, half_edge_b]
                            .map(|half_edge| half_edge.curve().clone()),
                    });
                    let vertices = Box::new(CoincidentHalfEdgeVertices {
                        vertices: [half_edge_a, half_edge_b].map(|half_edge| {
                            shell
                                .bounding_vertices_of_half_edge(half_edge)
                                .expect(
                                    "Expected half-edge to be part of shell",
                                )
                        }),
                    });

                    errors.push(
                        Self::CoincidentHalfEdgesAreNotSiblings {
                            boundaries,
                            curves,
                            vertices,
                            half_edge_a: half_edge_a.clone(),
                            half_edge_b: half_edge_b.clone(),
                        }
                        .into(),
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CoincidentHalfEdgeBoundaries {
    pub boundaries: [CurveBoundary<Point<1>>; 2],
}

impl fmt::Display for CoincidentHalfEdgeBoundaries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = &self.boundaries;

        if a != &b.reverse() {
            writeln!(
                f,
                "Boundaries don't match.\n\
                \tHalf-edge 1 has boundary `{a:?}`\n\
                \tHalf-edge 2 has boundary `{b:?}`\n\
                \t(expecting same boundary, but reversed)"
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CoincidentHalfEdgeCurves {
    pub curves: [Handle<Curve>; 2],
}

impl fmt::Display for CoincidentHalfEdgeCurves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = &self.curves;

        if a.id() != b.id() {
            writeln!(
                f,
                "Curves don't match.\n\
                \tHalf-edge 1 lies on {a:?}\n\
                \tHalf-edge 2 lies on {b:?}\n\
                \t(must be the same)"
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CurveCoordinateSystemMismatch {
    pub half_edge_a: Handle<HalfEdge>,
    pub half_edge_b: Handle<HalfEdge>,
    pub point_curve: Point<1>,
    pub point_a: Point<3>,
    pub point_b: Point<3>,
    pub distance: Scalar,
}

#[derive(Clone, Debug)]
pub struct CoincidentHalfEdgeVertices {
    pub vertices: [CurveBoundary<Vertex>; 2],
}

impl fmt::Display for CoincidentHalfEdgeVertices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = &self.vertices;

        if a != &b.clone().reverse() {
            writeln!(
                f,
                "Vertices don't match.\n\
                \tHalf-edge 1 is bounded by `{a:?}`\n\
                \tHalf-edge 2 is bounded by `{b:?}`\n\
                \t(expecting same vertices, but in reverse order)"
            )?;
        }

        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{Curve, Shell},
        operations::{
            build::BuildShell,
            insert::Insert,
            update::{
                UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion,
                UpdateShell,
            },
        },
        validate::{shell::ShellValidationError, Validate, ValidationError},
        Instance,
    };

    #[test]
    fn curve_coordinate_system_mismatch() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        let invalid = valid.shell.update_face(&valid.abc.face, |face| {
            [face
                .update_region(
                    |region, core| {
                        region
                            .update_exterior(|cycle| {
                                cycle
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(0),
                                        |edge, _| {
                                            [edge
                                                .update_path(|path| {
                                                    path.reverse()
                                                })
                                                .update_boundary(|boundary| {
                                                    boundary.reverse()
                                                })]
                                        },
                                        core,
                                    )
                                    .insert(&mut core.services)
                            })
                            .insert(&mut core.services)
                    },
                    &mut core,
                )
                .insert(&mut core.services)]
        });

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
    fn half_edge_has_no_sibling() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        let invalid = valid.shell.remove_face(&valid.abc.face);

        valid.shell.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Shell(
                ShellValidationError::HalfEdgeHasNoSibling { .. }
            )
        );

        Ok(())
    }

    #[test]
    fn coincident_half_edges_are_not_siblings() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        let invalid = valid.shell.update_face(&valid.abc.face, |face| {
            [face
                .update_region(
                    |region, core| {
                        region
                            .update_exterior(|cycle| {
                                cycle
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(0),
                                        |edge, core| {
                                            [edge.update_curve(
                                                |_, _| Curve::new(),
                                                core,
                                            )]
                                        },
                                        core,
                                    )
                                    .insert(&mut core.services)
                            })
                            .insert(&mut core.services)
                    },
                    &mut core,
                )
                .insert(&mut core.services)]
        });

        valid.shell.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::Shell(
                ShellValidationError::CoincidentHalfEdgesAreNotSiblings { .. }
            )
        );

        Ok(())
    }
}
