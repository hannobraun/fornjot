use fj_math::{Point, Scalar, Winding};

use crate::{
    objects::{Cycle, Face, Surface, SurfaceVertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_interior_winding(self, errors);
        FaceValidationError::check_vertex_positions(self, config, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// [`Surface`] of an interior [`Cycle`] doesn't match [`Face`]'s `Surface`
    #[error(
        "`Surface` of an interior `Cycle` doesn't match `Face`'s `Surface`\n\
        - `Surface` of the `Face`: {surface:#?}\n\
        - Invalid interior `Cycle`: {interior:#?}\n\
        - `Face`: {face:#?}"
    )]
    SurfaceMismatch {
        /// The surface of the [`Face`]
        surface: Handle<Surface>,

        /// The invalid interior cycle of the [`Face`]
        interior: Handle<Cycle>,

        /// The face
        face: Face,
    },

    /// Interior of [`Face`] has invalid winding; must be opposite of exterior
    #[error(
        "Interior of `Face` has invalid winding; must be opposite of exterior\n\
        - Winding of exterior cycle: {exterior_winding:#?}\n\
        - Winding of interior cycle: {interior_winding:#?}\n\
        - `Face`: {face:#?}"
    )]
    InvalidInteriorWinding {
        /// The winding of the [`Face`]'s exterior cycle
        exterior_winding: Winding,

        /// The winding of the invalid interior cycle
        interior_winding: Winding,

        /// The face
        face: Face,
    },

    /// Mismatch between [`SurfaceVertex`] and `GlobalVertex` positions
    #[error(
        "`SurfaceVertex` position doesn't match position of its global form\n\
        - Surface position: {surface_position:?}\n\
        - Surface position converted to global position: \
            {surface_position_as_global:?}\n\
        - Global position: {global_position:?}\n\
        - Distance between the positions: {distance}\n\
        - `SurfaceVertex`: {surface_vertex:#?}"
    )]
    VertexPositionMismatch {
        /// The position of the surface vertex
        surface_position: Point<2>,

        /// The surface position converted into a global position
        surface_position_as_global: Point<3>,

        /// The position of the global vertex
        global_position: Point<3>,

        /// The distance between the positions
        distance: Scalar,

        /// The surface vertex
        surface_vertex: Handle<SurfaceVertex>,
    },
}

impl FaceValidationError {
    fn check_interior_winding(face: &Face, errors: &mut Vec<ValidationError>) {
        let exterior_winding = face.exterior().winding();

        for interior in face.interiors() {
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Box::new(Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        face: face.clone(),
                    })
                    .into(),
                );
            }
        }
    }

    fn check_vertex_positions(
        face: &Face,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for cycle in face.all_cycles() {
            for half_edge in cycle.half_edges() {
                for surface_vertex in
                    [half_edge.start_vertex(), half_edge.end_vertex()]
                {
                    let surface_position_as_global = face
                        .surface()
                        .geometry()
                        .point_from_surface_coords(surface_vertex.position());
                    let global_position =
                        surface_vertex.global_form().position();

                    let distance = surface_position_as_global
                        .distance_to(&global_position);

                    if distance > config.identical_max_distance {
                        errors.push(
                            Box::new(Self::VertexPositionMismatch {
                                surface_position: surface_vertex.position(),
                                surface_position_as_global,
                                global_position,
                                distance,
                                surface_vertex: surface_vertex.clone(),
                            })
                            .into(),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::ext::ArrayExt;
    use fj_math::{Scalar, Vector};

    use crate::{
        algorithms::reverse::Reverse,
        builder::{CycleBuilder, FaceBuilder, HalfEdgeBuilder},
        insert::Insert,
        objects::{Cycle, Face, HalfEdge, SurfaceVertex},
        partial::{Partial, PartialCycle, PartialFace, PartialObject},
        services::Services,
        validate::Validate,
    };

    #[test]
    fn face_surface_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut face = PartialFace {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            face.exterior.write().update_as_polygon_from_points([
                [0., 0.],
                [3., 0.],
                [0., 3.],
            ]);
            face.add_interior().write().update_as_polygon_from_points([
                [1., 1.],
                [1., 2.],
                [2., 1.],
            ]);

            face.build(&mut services.objects)
        };
        let invalid = {
            let surface = services.objects.surfaces.xz_plane();

            let mut cycle = PartialCycle::default();
            cycle.update_as_polygon_from_points([[1., 1.], [1., 2.], [2., 1.]]);
            cycle.infer_vertex_positions_if_necessary(&surface.geometry());
            let cycle = cycle
                .build(&mut services.objects)
                .insert(&mut services.objects);

            let interiors = [cycle];
            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn face_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut face = PartialFace {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            face.exterior.write().update_as_polygon_from_points([
                [0., 0.],
                [3., 0.],
                [0., 3.],
            ]);
            face.add_interior().write().update_as_polygon_from_points([
                [1., 1.],
                [1., 2.],
                [2., 1.],
            ]);
            face.build(&mut services.objects)
        };
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services.objects))
                .collect::<Vec<_>>();

            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn surface_vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut face = PartialFace {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };

            let mut half_edge = face.exterior.write().add_half_edge();
            half_edge.write().update_as_circle_from_radius(1.);
            half_edge
                .write()
                .infer_vertex_positions_if_necessary(&surface.geometry());

            face.build(&mut services.objects)
        };
        let invalid = {
            let half_edge = {
                let half_edge = valid.exterior().half_edges().next().unwrap();

                let boundary = half_edge
                    .boundary()
                    .map(|point| point + Vector::from([Scalar::PI / 2.]));

                let mut surface_vertices =
                    [half_edge.start_vertex(), half_edge.end_vertex()]
                        .map(Clone::clone);

                let mut invalid = None;
                for surface_vertex in surface_vertices.each_mut_ext() {
                    let invalid = invalid.get_or_insert_with(|| {
                        SurfaceVertex::new(
                            [0., 1.],
                            surface_vertex.global_form().clone(),
                        )
                        .insert(&mut services.objects)
                    });
                    *surface_vertex = invalid.clone();
                }

                let [start_vertex, end_vertex] = surface_vertices;

                HalfEdge::new(
                    half_edge.curve(),
                    boundary,
                    start_vertex,
                    end_vertex,
                    half_edge.global_form().clone(),
                )
                .insert(&mut services.objects)
            };

            let exterior =
                Cycle::new([half_edge]).insert(&mut services.objects);

            Face::new(
                valid.surface().clone(),
                exterior,
                valid.interiors().cloned(),
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
