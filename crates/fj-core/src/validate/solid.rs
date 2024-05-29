use std::iter::repeat;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Solid, Vertex},
    validation::{
        checks::{MultipleReferencesToObject, ReferenceCounter},
        ValidationCheck,
    },
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
            .map(|(h, s)| {
                (
                    geometry.of_surface(s).point_from_surface_coords(
                        geometry.of_half_edge(&h).start_position(),
                    ),
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
        let mut faces = ReferenceCounter::new();

        solid.shells().iter().for_each(|s| {
            s.faces().into_iter().for_each(|f| {
                faces.count(f.clone(), s.clone());
            })
        });

        errors.extend(faces.multiples().map(Into::into));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        geometry::GlobalPath,
        operations::{
            build::{BuildFace, BuildHalfEdge, BuildSurface},
            insert::Insert,
        },
        topology::{Cycle, Face, HalfEdge, Region, Shell, Solid, Surface},
        validate::{Validate, ValidationError},
        Core,
    };

    #[test]
    fn should_find_face_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 1., 1.],
            &mut core,
        );

        let shared_face = Face::new(
            surface.clone(),
            Region::new(
                Cycle::new(vec![HalfEdge::circle(
                    [0., 0.],
                    1.,
                    surface,
                    &mut core,
                )])
                .insert(&mut core),
                vec![],
            )
            .insert(&mut core),
        )
        .insert(&mut core);

        let invalid_solid = Solid::new(vec![
            Shell::new(vec![shared_face.clone()]).insert(&mut core),
            Shell::new(vec![
                shared_face,
                Face::triangle(
                    [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.]],
                    &mut core,
                )
                .insert(&mut core)
                .face,
            ])
            .insert(&mut core),
        ])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid_solid,
            ValidationError::MultipleReferencesToFace(_)
        );

        let valid_solid = Solid::new(vec![]).insert(&mut core);
        valid_solid.validate_and_return_first_error(&core.layers.geometry)?;

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn should_find_region_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_region = Region::new(
            Cycle::new(vec![HalfEdge::circle(
                [0., 0.],
                1.,
                surface.clone(),
                &mut core,
            )])
            .insert(&mut core),
            vec![],
        )
        .insert(&mut core);

        let invalid_solid = Solid::new(vec![Shell::new(vec![
            Face::new(surface.clone(), shared_region.clone()).insert(&mut core),
            Face::new(surface, shared_region.clone()).insert(&mut core),
        ])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid_solid,
            ValidationError::MultipleReferencesToRegion(_)
        );

        let valid_solid = Solid::new(vec![]).insert(&mut core);
        valid_solid.validate_and_return_first_error(&core.layers.geometry)?;

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_cycle = Cycle::new(vec![HalfEdge::circle(
            [0., 0.],
            1.,
            surface.clone(),
            &mut core,
        )])
        .insert(&mut core);

        let invalid_solid = Solid::new(vec![Shell::new(vec![
            Face::new(
                surface.clone(),
                Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
            )
            .insert(&mut core),
            Face::new(
                surface,
                Region::new(shared_cycle, vec![]).insert(&mut core),
            )
            .insert(&mut core),
        ])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid_solid,
            ValidationError::MultipleReferencesToCycle(_)
        );

        let valid_solid = Solid::new(vec![]).insert(&mut core);
        valid_solid.validate_and_return_first_error(&core.layers.geometry)?;

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_edge =
            HalfEdge::circle([0., 0.], 1., surface.clone(), &mut core);

        let invalid_solid = Solid::new(vec![Shell::new(vec![Face::new(
            surface,
            Region::new(
                Cycle::new(vec![shared_edge.clone()]).insert(&mut core),
                vec![Cycle::new(vec![shared_edge.clone()]).insert(&mut core)],
            )
            .insert(&mut core),
        )
        .insert(&mut core)])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid_solid,
            ValidationError::MultipleReferencesToHalfEdge(_)
        );

        let valid_solid = Solid::new(vec![]).insert(&mut core);
        valid_solid.validate_and_return_first_error(&core.layers.geometry)?;

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }
}
