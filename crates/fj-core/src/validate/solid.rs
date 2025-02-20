use std::iter::repeat;

use crate::{
    geometry::{Geometry, repr::tri_mesh::convert_point_surface_to_global},
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Shell, Solid, Vertex},
    validation::{ValidationCheck, checks::MultipleReferencesToObject},
};
use fj_math::Point;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Solid {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            MultipleReferencesToObject::<Face, Shell>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        errors.extend(
            MultipleReferencesToObject::<Region, Face>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        errors.extend(
            MultipleReferencesToObject::<Cycle, Region>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        errors.extend(
            MultipleReferencesToObject::<HalfEdge, Cycle>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        SolidValidationError::check_vertices(self, geometry, config, errors);
    }
}

/// [`Solid`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SolidValidationError {
    /// [`Solid`] contains vertices that are coincident, but not identical
    #[error(
        "Solid contains Vertices that are coincident but not identical\n
        Vertex 1: {vertex_a:#?} ({position_a:?})
        Vertex 2: {vertex_b:#?} ({position_b:?})"
    )]
    DistinctVerticesCoincide {
        /// The first vertex
        vertex_a: Handle<Vertex>,

        /// The second vertex
        vertex_b: Handle<Vertex>,

        /// Position of first vertex
        position_a: Point<3>,

        /// Position of second vertex
        position_b: Point<3>,
    },

    /// [`Solid`] contains vertices that are identical, but do not coincide
    #[error(
        "Solid contains Vertices that are identical but do not coincide\n
        Vertex 1: {vertex_a:#?} ({position_a:?})
        Vertex 2: {vertex_b:#?} ({position_b:?})"
    )]
    IdenticalVerticesNotCoincident {
        /// The first vertex
        vertex_a: Handle<Vertex>,

        /// The second vertex
        vertex_b: Handle<Vertex>,

        /// Position of first vertex
        position_a: Point<3>,

        /// Position of second vertex
        position_b: Point<3>,
    },
}

impl SolidValidationError {
    fn check_vertices(
        solid: &Solid,
        geometry: &Geometry,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let vertices: Vec<(Point<3>, Handle<Vertex>)> = solid
            .shells()
            .iter()
            .flat_map(|s| s.faces())
            .flat_map(|face| {
                face.region()
                    .all_cycles()
                    .flat_map(|cycle| cycle.half_edges().iter().cloned())
                    .zip(repeat(face.surface()))
            })
            .filter_map(|(h, s)| {
                let Some(local_curve_geometry) =
                    geometry.of_curve(h.curve()).unwrap().local_on(s)
                else {
                    // If the curve geometry has no local definition,
                    // there's nothing we can check.
                    return None;
                };

                Some((
                    convert_point_surface_to_global(
                        &geometry.of_surface_2(s).unwrap().generator,
                        local_curve_geometry.path.point_from_path_coords(
                            geometry
                                .of_vertex(h.start_vertex())
                                .unwrap()
                                .local_on(h.curve())
                                .unwrap()
                                .position,
                        ),
                        config.tolerance,
                        geometry,
                    ),
                    h.start_vertex().clone(),
                ))
            })
            .collect();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (position_a, vertex_a) in &vertices {
            for (position_b, vertex_b) in &vertices {
                let vertices_are_identical = vertex_a.id() == vertex_b.id();
                let vertices_are_not_identical = !vertices_are_identical;

                let too_far_to_be_identical = position_a
                    .distance_to(position_b)
                    > config.identical_max_distance;
                let too_close_to_be_distinct = position_a
                    .distance_to(position_b)
                    < config.distinct_min_distance;

                if vertices_are_identical && too_far_to_be_identical {
                    errors.push(
                        Self::IdenticalVerticesNotCoincident {
                            vertex_a: vertex_a.clone(),
                            vertex_b: vertex_b.clone(),
                            position_a: *position_a,
                            position_b: *position_b,
                        }
                        .into(),
                    )
                }

                if vertices_are_not_identical && too_close_to_be_distinct {
                    errors.push(
                        Self::DistinctVerticesCoincide {
                            vertex_a: vertex_a.clone(),
                            vertex_b: vertex_b.clone(),
                            position_a: *position_a,
                            position_b: *position_b,
                        }
                        .into(),
                    )
                }
            }
        }
    }
}
