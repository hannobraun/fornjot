use fj_math::{Point, Scalar};

use crate::{
    geometry::{Geometry, SurfaceGeom},
    queries::{
        AllHalfEdgesWithSurface, BoundingVerticesOfHalfEdge, SiblingOfHalfEdge,
    },
    storage::Handle,
    topology::{HalfEdge, Shell},
    validation::{
        checks::{
            CoincidentHalfEdgesAreNotSiblings, CurveGeometryMismatch,
            HalfEdgeHasNoSibling,
        },
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
        errors.extend(
            ShellValidationError::check_half_edge_coincidence(
                self, geometry, config,
            )
            .map(Into::into),
        );
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {}

impl ShellValidationError {
    /// Check that non-sibling half-edges are not coincident
    fn check_half_edge_coincidence(
        shell: &Shell,
        geometry: &Geometry,
        config: &ValidationConfig,
    ) -> impl Iterator<Item = CoincidentHalfEdgesAreNotSiblings> {
        let mut errors = Vec::new();

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
                    let boundaries =
                        [half_edge_a, half_edge_b].map(|half_edge| {
                            geometry.of_half_edge(half_edge).boundary
                        });
                    let curves = [half_edge_a, half_edge_b]
                        .map(|half_edge| half_edge.curve().clone());
                    let vertices =
                        [half_edge_a, half_edge_b].map(|half_edge| {
                            shell
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
        validate::{Validate, ValidationError},
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
            ValidationError::CoincidentHalfEdgesAreNotSiblings { .. }
        );

        Ok(())
    }
}
