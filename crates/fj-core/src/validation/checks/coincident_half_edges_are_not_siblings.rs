use std::fmt;

use fj_math::{Point, Scalar};

use crate::{
    geometry::{CurveBoundary, Geometry, SurfaceGeom},
    queries::{
        AllHalfEdgesWithSurface, BoundingVerticesOfHalfEdge, SiblingOfHalfEdge,
    },
    storage::Handle,
    topology::{Curve, HalfEdge, Shell, Surface, Vertex},
    validation::ValidationCheck,
};

/// A [`Shell`] contains two [`HalfEdge`]s that are coincident but not siblings
///
/// Coincident half-edges must reference the same curve, and have the same
/// boundaries on that curve. This provides clear, topological information,
/// which is important to handle the shell geometry in a robust way.
#[derive(Clone, Debug, thiserror::Error)]
pub struct CoincidentHalfEdgesAreNotSiblings {
    /// The boundaries of the half-edges
    pub boundaries: [CurveBoundary<Point<1>>; 2],

    /// The curves of the half-edges
    pub curves: [Handle<Curve>; 2],

    /// The vertices of the half-edges
    pub vertices: [CurveBoundary<Vertex>; 2],

    /// The first half-edge
    pub half_edge_a: Handle<HalfEdge>,

    /// The second half-edge
    pub half_edge_b: Handle<HalfEdge>,
}

impl fmt::Display for CoincidentHalfEdgesAreNotSiblings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "`Shell` contains `HalfEdge`s that are coincident but are not \
            siblings",
        )?;

        {
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
        }

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
            Half-edge 2: {:#?}",
            self.half_edge_a, self.half_edge_b,
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

                if object.are_siblings(half_edge_a, half_edge_b, geometry) {
                    // If the half-edges are siblings, they are allowed to be
                    // coincident. Must be, in fact. There's another validation
                    // check that takes care of that.
                    continue;
                }

                // If all points on distinct curves are within
                // `distinct_min_distance`, that's a problem.
                if distances(
                    half_edge_a.clone(),
                    surface_a,
                    half_edge_b.clone(),
                    surface_b,
                    geometry,
                )
                .all(|d| d < config.distinct_min_distance)
                {
                    let boundaries =
                        [half_edge_a, half_edge_b].map(|half_edge| {
                            geometry.of_half_edge(half_edge).boundary
                        });
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
                        boundaries,
                        curves,
                        vertices,
                        half_edge_a: half_edge_a.clone(),
                        half_edge_b: half_edge_b.clone(),
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
    half_edge_a: Handle<HalfEdge>,
    surface_a: &Handle<Surface>,
    half_edge_b: Handle<HalfEdge>,
    surface_b: &Handle<Surface>,
    geometry: &Geometry,
) -> impl Iterator<Item = Scalar> {
    fn sample(
        percent: f64,
        (half_edge, surface): (&Handle<HalfEdge>, &SurfaceGeom),
        geometry: &Geometry,
    ) -> Point<3> {
        let [start, end] = geometry.of_half_edge(half_edge).boundary.inner;
        let path_coords = start + (end - start) * percent;
        let surface_coords = geometry
            .of_half_edge(half_edge)
            .path
            .point_from_path_coords(path_coords);
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
        let sample1 = sample(
            percent,
            (&half_edge_a, geometry.of_surface(surface_a)),
            geometry,
        );
        let sample2 = sample(
            1.0 - percent,
            (&half_edge_b, geometry.of_surface(surface_b)),
            geometry,
        );
        distances.push(sample1.distance_to(&sample2))
    }
    distances.into_iter()
}

#[cfg(test)]
mod tests {
    use crate::{
        operations::{
            build::BuildShell,
            geometry::{UpdateCurveGeometry, UpdateHalfEdgeGeometry},
            insert::Insert,
            update::{
                UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion,
                UpdateShell,
            },
        },
        topology::{Curve, Shell},
        validation::{
            checks::CoincidentHalfEdgesAreNotSiblings, ValidationCheck,
        },
        Core,
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

                                        [half_edge
                                            .update_curve(|_, _| curve, core)
                                            .insert(core)
                                            .set_geometry(
                                                *core
                                                    .layers
                                                    .geometry
                                                    .of_half_edge(half_edge),
                                                &mut core.layers.geometry,
                                            )]
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
