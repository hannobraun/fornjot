use std::iter::repeat;

use crate::{
    objects::{Solid, Vertex},
    storage::Handle,
    validate_references,
};
use fj_math::Point;

use super::{
    references::{ReferenceCountError, ReferenceCounter},
    Validate, ValidationConfig, ValidationError,
};

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

    /// Object within solid referenced by more than one other object
    #[error("Object within solid referenced by more than one other Object")]
    MultipleReferences(#[from] ReferenceCountError),
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
        let mut referenced_regions = ReferenceCounter::new();
        let mut referenced_faces = ReferenceCounter::new();
        let mut referenced_edges = ReferenceCounter::new();
        let mut referenced_cycles = ReferenceCounter::new();

        solid.shells().iter().for_each(|s| {
            s.faces().into_iter().for_each(|f| {
                referenced_faces.add_count(f.clone());
                referenced_regions.add_count(f.region().clone());
                f.region().all_cycles().for_each(|c| {
                    referenced_cycles.add_count(c.clone());
                    c.half_edges().into_iter().for_each(|e| {
                        referenced_edges.add_count(e.clone());
                    })
                })
            })
        });

        validate_references!(
            errors, SolidValidationError;
            referenced_regions, ReferenceCountError::Region;
            referenced_faces, ReferenceCountError::Face;
            referenced_edges, ReferenceCountError::HalfEdge;
            referenced_cycles, ReferenceCountError::Cycle;
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        geometry::{GlobalPath, SurfaceGeometry},
        objects::{Cycle, Face, HalfEdge, Region, Shell, Solid, Surface},
        operations::{
            build::{BuildFace, BuildHalfEdge},
            insert::Insert,
        },
        services::Services,
        validate::{
            references::ReferenceCountError, SolidValidationError, Validate,
            ValidationError,
        },
    };

    #[test]
    fn should_find_face_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let shared_face = Face::new(
            Surface::new(SurfaceGeometry {
                u: GlobalPath::circle_from_radius(1.),
                v: [0., 1., 1.].into(),
            })
            .insert(&mut services),
            Region::new(
                Cycle::new(vec![HalfEdge::circle([0., 0.], 1., &mut services)
                    .insert(&mut services)])
                .insert(&mut services),
                vec![],
                None,
            )
            .insert(&mut services),
        )
        .insert(&mut services);

        let invalid_solid = Solid::new(vec![
            Shell::new(vec![shared_face.clone()]).insert(&mut services),
            Shell::new(vec![
                shared_face,
                Face::triangle(
                    [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.]],
                    &mut services,
                )
                .insert(&mut services)
                .face,
            ])
            .insert(&mut services),
        ])
        .insert(&mut services);

        assert_contains_err!(
            invalid_solid,
            ValidationError::Solid(SolidValidationError::MultipleReferences(
                ReferenceCountError::Face
            ))
        );

        let valid_solid = Solid::new(vec![]).insert(&mut services);
        valid_solid.validate_and_return_first_error()?;

        services.validation.errors.clear();

        Ok(())
    }

    #[test]
    fn should_find_region_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let shared_region = Region::new(
            Cycle::new(vec![HalfEdge::circle([0., 0.], 1., &mut services)
                .insert(&mut services)])
            .insert(&mut services),
            vec![],
            None,
        )
        .insert(&mut services);

        let invalid_solid = Solid::new(vec![Shell::new(vec![
            Face::new(
                Surface::new(SurfaceGeometry {
                    u: GlobalPath::circle_from_radius(1.),
                    v: [0., 1., 1.].into(),
                })
                .insert(&mut services),
                shared_region.clone(),
            )
            .insert(&mut services),
            Face::new(
                Surface::new(SurfaceGeometry {
                    u: GlobalPath::circle_from_radius(1.),
                    v: [0., 0., 1.].into(),
                })
                .insert(&mut services),
                shared_region.clone(),
            )
            .insert(&mut services),
        ])
        .insert(&mut services)])
        .insert(&mut services);

        assert_contains_err!(
            invalid_solid,
            ValidationError::Solid(SolidValidationError::MultipleReferences(
                ReferenceCountError::Region
            ))
        );

        let valid_solid = Solid::new(vec![]).insert(&mut services);
        valid_solid.validate_and_return_first_error()?;

        services.validation.errors.clear();

        Ok(())
    }

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let shared_cycle =
            Cycle::new(vec![HalfEdge::circle([0., 0.], 1., &mut services)
                .insert(&mut services)])
            .insert(&mut services);

        let invalid_solid = Solid::new(vec![Shell::new(vec![
            Face::new(
                Surface::new(SurfaceGeometry {
                    u: GlobalPath::circle_from_radius(1.),
                    v: [0., 1., 1.].into(),
                })
                .insert(&mut services),
                Region::new(shared_cycle.clone(), vec![], None)
                    .insert(&mut services),
            )
            .insert(&mut services),
            Face::new(
                Surface::new(SurfaceGeometry {
                    u: GlobalPath::circle_from_radius(1.),
                    v: [0., 0., 1.].into(),
                })
                .insert(&mut services),
                Region::new(shared_cycle, vec![], None).insert(&mut services),
            )
            .insert(&mut services),
        ])
        .insert(&mut services)])
        .insert(&mut services);

        assert_contains_err!(
            invalid_solid,
            ValidationError::Solid(SolidValidationError::MultipleReferences(
                ReferenceCountError::Cycle
            ))
        );

        let valid_solid = Solid::new(vec![]).insert(&mut services);
        valid_solid.validate_and_return_first_error()?;

        services.validation.errors.clear();

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let shared_edge =
            HalfEdge::circle([0., 0.], 1., &mut services).insert(&mut services);

        let invalid_solid = Solid::new(vec![Shell::new(vec![Face::new(
            Surface::new(SurfaceGeometry {
                u: GlobalPath::circle_from_radius(1.),
                v: [0., 0., 1.].into(),
            })
            .insert(&mut services),
            Region::new(
                Cycle::new(vec![shared_edge.clone()]).insert(&mut services),
                vec![
                    Cycle::new(vec![shared_edge.clone()]).insert(&mut services)
                ],
                None,
            )
            .insert(&mut services),
        )
        .insert(&mut services)])
        .insert(&mut services)])
        .insert(&mut services);

        assert_contains_err!(
            invalid_solid,
            ValidationError::Solid(SolidValidationError::MultipleReferences(
                ReferenceCountError::HalfEdge
            ))
        );

        let valid_solid = Solid::new(vec![]).insert(&mut services);
        valid_solid.validate_and_return_first_error()?;

        services.validation.errors.clear();

        Ok(())
    }
}
