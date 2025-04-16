use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar};

use crate::{
    geometry::{
        CurveBoundary, Geometry,
        repr::tri_mesh::convert_point_surface_to_global,
    },
    queries::{
        AllHalfEdgesWithSurface, BoundingVerticesOfHalfEdge, CycleOfHalfEdge,
        SiblingOfHalfEdge,
    },
    storage::Handle,
    topology::{Curve, HalfEdge, Shell, Surface, Vertex},
    validation::ValidationCheck,
};

/// A [`Shell`] contains two [`HalfEdge`]s that are coincident but not siblings
///
/// Coincident half-edges must reference the same curve, and must have opposite
/// start and end vertices (i.e. the start vertex of one must be the end vertex
/// of the other, respectively).
#[derive(Clone, Debug, thiserror::Error)]
pub struct CoincidentHalfEdgesAreNotSiblings {
    /// The curves of the half-edges
    pub curves: [Handle<Curve>; 2],

    /// The vertices of the half-edges
    pub vertices: [CurveBoundary<Vertex>; 2],

    /// The first half-edge
    pub half_edge_a: Handle<HalfEdge>,

    /// The second half-edge
    pub half_edge_b: Handle<HalfEdge>,

    /// The points on the half-edges that were checked
    pub points: Vec<[Point<3>; 2]>,

    /// The distances between the points on the half-edges that were checked
    pub distances: Vec<Scalar>,
}

impl fmt::Display for CoincidentHalfEdgesAreNotSiblings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "`Shell` contains `HalfEdge`s that are coincident but are not \
            siblings",
        )?;

        {
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
        }

        {
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
        }

        write!(
            f,
            "Half-edge 1: {:#?}\n\
            Half-edge 2: {:#?}\n\
            Points: {:#?}\n\
            Distances: {:#?}",
            self.half_edge_a, self.half_edge_b, self.points, self.distances
        )?;

        Ok(())
    }
}

impl ValidationCheck<Shell> for CoincidentHalfEdgesAreNotSiblings {
    fn check<'r>(
        object: &'r Shell,
        geometry: &'r crate::geometry::Geometry,
        config: &'r crate::validation::ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut errors = Vec::new();

        let edges_and_surfaces =
            object.all_half_edges_with_surface().collect::<Vec<_>>();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (half_edge_a, surface_a) in &edges_and_surfaces {
            for (half_edge_b, surface_b) in &edges_and_surfaces {
                // No need to check a half-edge against itself.
                if half_edge_a.id() == half_edge_b.id() {
                    continue;
                }

                if object.are_siblings(half_edge_a, half_edge_b) {
                    // If the half-edges are siblings, they are allowed to be
                    // coincident. Must be, in fact. There's another validation
                    // check that takes care of that.
                    continue;
                }

                let Some(points_and_distances) = distances(
                    (
                        half_edge_a.clone(),
                        object
                            .find_cycle_of_half_edge(half_edge_a)
                            .unwrap()
                            .half_edges()
                            .after(half_edge_a)
                            .unwrap()
                            .start_vertex(),
                        surface_a,
                    ),
                    (
                        half_edge_b.clone(),
                        object
                            .find_cycle_of_half_edge(half_edge_b)
                            .unwrap()
                            .half_edges()
                            .after(half_edge_b)
                            .unwrap()
                            .start_vertex(),
                        surface_b,
                    ),
                    config.tolerance,
                    geometry,
                ) else {
                    // The geometry to compute the distances is not available,
                    // hence these half-edges can't be coincident.
                    continue;
                };

                let (points, distances): (Vec<_>, Vec<_>) =
                    points_and_distances.into_iter().unzip();

                // If all points on distinct curves are within
                // `distinct_min_distance`, that's a problem.
                if distances.iter().all(|d| *d < config.distinct_min_distance) {
                    let curves = [half_edge_a, half_edge_b]
                        .map(|half_edge| half_edge.curve().clone());
                    let vertices =
                        [half_edge_a, half_edge_b].map(|half_edge| {
                            object
                                .bounding_vertices_of_half_edge(half_edge)
                                .expect(
                                    "Expected half-edge to be part of shell",
                                )
                        });

                    errors.push(CoincidentHalfEdgesAreNotSiblings {
                        curves,
                        vertices,
                        half_edge_a: half_edge_a.clone(),
                        half_edge_b: half_edge_b.clone(),
                        points,
                        distances,
                    })
                }
            }
        }

        errors.into_iter()
    }
}

/// Sample two edges at various (currently 3) points in 3D along them.
///
/// Returns an [`Iterator`] of the distance at each sample.
fn distances(
    (half_edge_a, end_vertex_a, surface_a): (
        Handle<HalfEdge>,
        &Handle<Vertex>,
        &Handle<Surface>,
    ),
    (half_edge_b, end_vertex_b, surface_b): (
        Handle<HalfEdge>,
        &Handle<Vertex>,
        &Handle<Surface>,
    ),
    tolerance: Tolerance,
    geometry: &Geometry,
) -> Option<Vec<([Point<3>; 2], Scalar)>> {
    fn sample(
        percent: f64,
        half_edge: &Handle<HalfEdge>,
        end_vertex: &Handle<Vertex>,
        surface: &Handle<Surface>,
        tolerance: Tolerance,
        geometry: &Geometry,
    ) -> Option<Point<3>> {
        let [start, end] = [
            geometry
                .of_vertex(half_edge.start_vertex())
                .unwrap()
                .local_on(half_edge.curve())
                .unwrap()
                .position,
            geometry
                .of_vertex(end_vertex)
                .unwrap()
                .local_on(half_edge.curve())
                .unwrap()
                .position,
        ];
        let path_coords = start + (end - start) * percent;
        let path = geometry
            .of_curve(half_edge.curve())?
            .local_on(surface)?
            .path;
        let surface_coords = path.point_from_path_coords(path_coords);
        Some(convert_point_surface_to_global(
            &geometry.of_surface_2(surface).unwrap().generator,
            surface_coords,
            tolerance,
            geometry,
        ))
    }

    // Three samples (start, middle, end), are enough to detect weather lines
    // and circles match. If we were to add more complicated curves, this might
    // need to change.
    let sample_count = 3;
    let step = 1.0 / (sample_count as f64 - 1.0);

    let mut distances = Vec::new();
    for i in 0..sample_count {
        let percent = i as f64 * step;
        let sample1 = sample(
            percent,
            &half_edge_a,
            end_vertex_a,
            surface_a,
            tolerance,
            geometry,
        )?;
        let sample2 = sample(
            1.0 - percent,
            &half_edge_b,
            end_vertex_b,
            surface_b,
            tolerance,
            geometry,
        )?;
        distances.push(([sample1, sample2], sample1.distance_to(&sample2)))
    }
    Some(distances)
}

#[cfg(test)]
mod tests {
    use crate::{
        Core,
        operations::{
            build::BuildShell,
            geometry::UpdateCurveGeometry,
            insert::Insert,
            update::{
                UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion,
                UpdateShell,
            },
        },
        topology::{Curve, Shell},
        validation::{
            ValidationCheck, checks::CoincidentHalfEdgesAreNotSiblings,
        },
    };

    #[test]
    fn coincident_half_edges_are_not_siblings() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        CoincidentHalfEdgesAreNotSiblings::check_and_return_first_error(
            &valid.shell,
            &core.layers.geometry,
        )?;

        let invalid = valid.shell.update_face(
            &valid.abc.face,
            |face, core| {
                [face.update_region(
                    |region, core| {
                        region.update_exterior(
                            |cycle, core| {
                                cycle.update_half_edge(
                                    cycle.half_edges().nth_circular(0),
                                    |half_edge, core| {
                                        let curve = Curve::new()
                                            .insert(core)
                                            .copy_geometry_from(
                                                half_edge.curve(),
                                                &mut core.layers.geometry,
                                            );

                                        let start_vertex =
                                            half_edge.start_vertex();
                                        let end_vertex = cycle
                                            .half_edges()
                                            .after(half_edge)
                                            .unwrap()
                                            .start_vertex();

                                        core.layers.geometry.define_vertex(
                                            start_vertex.clone(),
                                            curve.clone(),
                                            core.layers
                                                .geometry
                                                .of_vertex(start_vertex)
                                                .unwrap()
                                                .local_on(half_edge.curve())
                                                .unwrap()
                                                .clone(),
                                        );
                                        core.layers.geometry.define_vertex(
                                            end_vertex.clone(),
                                            curve.clone(),
                                            core.layers
                                                .geometry
                                                .of_vertex(end_vertex)
                                                .unwrap()
                                                .local_on(half_edge.curve())
                                                .unwrap()
                                                .clone(),
                                        );

                                        [half_edge
                                            .update_curve(|_, _| curve, core)
                                            .insert(core)]
                                    },
                                    core,
                                )
                            },
                            core,
                        )
                    },
                    core,
                )]
            },
            &mut core,
        );
        assert!(
            CoincidentHalfEdgesAreNotSiblings::check_and_return_first_error(
                &invalid,
                &core.layers.geometry,
            )
            .is_err()
        );

        Ok(())
    }
}
