use std::iter::repeat;

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Solid, Vertex},
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
        SolidValidationError::check_vertices(self, config, errors);
        SolidValidationError::check_object_references(self, config, errors);
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
    /// [`Region`] referenced by multiple faces
    #[error("Region referenced by multiple faces")]
    RegionMultipleReferences,
    /// [`Face`] referenced by multiple shells
    #[error("Face referenced by multiple shells")]
    FaceMultipleReferences,
    /// [`HalfEdge`] referenced by more than one [`Cycle`]
    #[error("[`HalfEdge`] referenced by more than one [`Cycle`]")]
    HalfEdgeMultipleReferences,
    /// [`Cycle`] referenced by more than one [`Region`]
    #[error("[`Cycle`] referenced by more than one [`Region`]")]
    CycleMultipleReferences,
}

impl SolidValidationError {
    fn check_vertices(
        solid: &Solid,
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

    fn check_object_references(
        solid: &Solid,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        // todo: store referencing objects instead of just a reference count so that we can surface
        // them in the error message
        let mut referenced_regions =
            std::collections::HashMap::<Handle<Region>, i32>::new();
        let mut referenced_faces =
            std::collections::HashMap::<Handle<Face>, i32>::new();
        let mut referenced_edges =
            std::collections::HashMap::<Handle<HalfEdge>, i32>::new();
        let mut referenced_cycles =
            std::collections::HashMap::<Handle<Cycle>, i32>::new();

        referenced_cycles.iter().for_each(|(_, count)| {
            if count > &1 {
                errors.push(Self::CycleMultipleReferences.into());
            }
        });
        referenced_edges.iter().for_each(|(_, count)| {
            if count > &1 {
                errors.push(Self::HalfEdgeMultipleReferences.into());
            }
        });

        solid.shells().iter().for_each(|s| {
            s.faces().into_iter().for_each(|f| {
                referenced_faces.insert(f.clone(), {
                    if let Some(count) = referenced_faces.get(f) {
                        count + 1
                    } else {
                        1
                    }
                });

                referenced_regions.insert(f.region().clone(), {
                    if let Some(count) = referenced_regions.get(f.region()) {
                        count + 1
                    } else {
                        1
                    }
                });
                f.region().all_cycles().for_each(|c| {
                    referenced_cycles.insert(c.clone(), {
                        if let Some(count) = referenced_cycles.get(c) {
                            count + 1
                        } else {
                            1
                        }
                    });
                    c.half_edges().into_iter().for_each(|e| {
                        referenced_edges.insert(e.clone(), {
                            if let Some(count) = referenced_edges.get(e) {
                                count + 1
                            } else {
                                1
                            }
                        });
                    })
                })
            })
        });

        referenced_faces.iter().for_each(|(_, count)| {
            if count > &1 {
                errors.push(Self::FaceMultipleReferences.into());
            }
        });
        referenced_regions.iter().for_each(|(_, count)| {
            if count > &1 {
                errors.push(Self::RegionMultipleReferences.into());
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_find_face_multiple_references() {
        unimplemented!();
    }

    #[test]
    fn should_find_region_multiple_references() {
        unimplemented!();
    }

    #[test]
    fn should_find_cycle_multiple_references() {
        unimplemented!();
    }

    #[test]
    fn should_find_half_edge_multiple_references() {
        unimplemented!();
    }
}
