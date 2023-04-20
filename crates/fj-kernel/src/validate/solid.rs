use std::iter::repeat;

use crate::{
    objects::{Solid, Vertex},
    storage::Handle,
};
use fj_math::Point;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Solid {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        SolidValidationError::check_vertices(self, config, errors)
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
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let vertices: Vec<(Point<3>, Handle<Vertex>)> = solid
            .shells()
            .flat_map(|s| s.faces())
            .flat_map(|face| {
                face.all_cycles()
                    .flat_map(|cycle| cycle.half_edges().cloned())
                    .zip(repeat(face.surface().geometry()))
            })
            .map(|(h, s)| {
                (
                    s.point_from_surface_coords(h.start_position()),
                    h.start_vertex().clone(),
                )
            })
            .collect();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for (position_a, vertex_a) in &vertices {
            for (position_b, vertex_b) in &vertices {
                let vertices_are_identical = vertex_a.id() == vertex_b.id();
                let too_far_to_be_identical = position_a
                    .distance_to(position_b)
                    > config.identical_max_distance;

                match vertices_are_identical {
                    true => {
                        if too_far_to_be_identical {
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
                    }
                    false => {
                        if position_a.distance_to(position_b)
                            < config.distinct_min_distance
                        {
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
    }
}
