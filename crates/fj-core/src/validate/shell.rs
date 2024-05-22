use std::fmt;

use fj_math::{Point, Scalar};

use crate::{
    geometry::{CurveBoundary, Geometry, SurfaceGeom},
    queries::{
        AllHalfEdgesWithSurface, BoundingVerticesOfHalfEdge, SiblingOfHalfEdge,
    },
    storage::Handle,
    topology::{Curve, HalfEdge, Shell, Vertex},
    validation::{
        checks::{CurveGeometryMismatch, HalfEdgeHasNoSibling},
        ValidationCheck,
    },
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            CurveGeometryMismatch::check(self, geometry, config)
                .map(Into::into),
        );
        errors.extend(
            HalfEdgeHasNoSibling::check(self, geometry, config).map(Into::into),
        );
        ShellValidationError::check_half_edge_coincidence(
            self, geometry, config, errors,
        );
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {
    /// [`Shell`] contains half-edges that are coincident, but aren't siblings
    #[error(transparent)]
    CoincidentHalfEdgesAreNotSiblings(CoincidentHalfEdgesAreNotSiblings),
}

impl ShellValidationError {
    /// Check that non-sibling half-edges are not coincident
    fn check_half_edge_coincidence(
        shell: &Shell,
        geometry: &Geometry,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let edges_and_surfaces =
            shell.all_half_edges_with_surface().collect::<Vec<_>>();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (half_edge_a, surface_a) in &edges_and_surfaces {
            for (half_edge_b, surface_b) in &edges_and_surfaces {
                // No need to check a half-edge against itself.
                if half_edge_a.id() == half_edge_b.id() {
                    continue;
                }

                if shell.are_siblings(half_edge_a, half_edge_b, geometry) {
                    // If the half-edges are siblings, they are allowed to be
                    // coincident. Must be, in fact. There's another validation
                    // check that takes care of that.
                    continue;
                }

                // If all points on distinct curves are within
                // `distinct_min_distance`, that's a problem.
                if distances(
                    half_edge_a.clone(),
                    geometry.of_surface(surface_a),
                    half_edge_b.clone(),
                    geometry.of_surface(surface_b),
                    geometry,
                )
                .all(|d| d < config.distinct_min_distance)
                {
                    let boundaries = Box::new(CoincidentHalfEdgeBoundaries {
                        boundaries: [half_edge_a, half_edge_b].map(
                            |half_edge| {
                                geometry.of_half_edge(half_edge).boundary
                            },
                        ),
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
                        Self::CoincidentHalfEdgesAreNotSiblings(
                            CoincidentHalfEdgesAreNotSiblings {
                                boundaries,
                                curves,
                                vertices,
                                half_edge_a: half_edge_a.clone(),
                                half_edge_b: half_edge_b.clone(),
                            },
                        )
                        .into(),
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[error(
    "`Shell` contains `HalfEdge`s that are coincident but are not siblings\n\
    {boundaries}\
    {curves}\
    {vertices}\
    Half-edge 1: {half_edge_a:#?}\n\
    Half-edge 2: {half_edge_b:#?}"
)]
pub struct CoincidentHalfEdgesAreNotSiblings {
    /// The boundaries of the half-edges
    pub boundaries: Box<CoincidentHalfEdgeBoundaries>,

    /// The curves of the half-edges
    pub curves: Box<CoincidentHalfEdgeCurves>,

    /// The vertices of the half-edges
    pub vertices: Box<CoincidentHalfEdgeVertices>,

    /// The first half-edge
    pub half_edge_a: Handle<HalfEdge>,

    /// The second half-edge
    pub half_edge_b: Handle<HalfEdge>,
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
    surface_a: &SurfaceGeom,
    edge_b: Handle<HalfEdge>,
    surface_b: &SurfaceGeom,
    geometry: &Geometry,
) -> impl Iterator<Item = Scalar> {
    fn sample(
        percent: f64,
        (edge, surface): (&Handle<HalfEdge>, &SurfaceGeom),
        geometry: &Geometry,
    ) -> Point<3> {
        let [start, end] = geometry.of_half_edge(edge).boundary.inner;
        let path_coords = start + (end - start) * percent;
        let surface_coords = geometry
            .of_half_edge(edge)
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
        let sample1 = sample(percent, (&edge_a, surface_a), geometry);
        let sample2 = sample(1.0 - percent, (&edge_b, surface_b), geometry);
        distances.push(sample1.distance_to(&sample2))
    }
    distances.into_iter()
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
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
        validate::{shell::ShellValidationError, Validate, ValidationError},
        Core,
    };

    #[test]
    fn coincident_half_edges_are_not_siblings() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
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

        valid
            .shell
            .validate_and_return_first_error(&core.layers.geometry)?;
        assert_contains_err!(
            core,
            invalid,
            ValidationError::Shell(
                ShellValidationError::CoincidentHalfEdgesAreNotSiblings { .. }
            )
        );

        Ok(())
    }
}
