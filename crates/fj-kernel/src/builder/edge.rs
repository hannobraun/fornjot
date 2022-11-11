use fj_math::{Point, Scalar};
use iter_fixed::IntoIteratorFixed;

use crate::{
    insert::Insert,
    objects::{
        Curve, GlobalVertex, Objects, Surface, Vertex,
        VerticesInNormalizedOrder,
    },
    partial::{
        HasPartial, MaybePartial, PartialCurve, PartialGlobalEdge,
        PartialHalfEdge, PartialSurfaceVertex,
    },
    storage::Handle,
    validate::ValidationError,
};

use super::{CurveBuilder, GlobalVertexBuilder};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder: Sized {
    /// Update the partial half-edge with the given back vertex
    fn with_back_vertex(self, back: impl Into<MaybePartial<Vertex>>) -> Self;

    /// Update the partial half-edge with the given front vertex
    fn with_front_vertex(self, front: impl Into<MaybePartial<Vertex>>) -> Self;

    /// Update partial half-edge as a circle, from the given radius
    ///
    /// # Implementation Note
    ///
    /// In principle, only the `build` method should take a reference to
    /// [`Objects`]. As of this writing, this method is the only one that
    /// deviates from that. I couldn't think of a way to do it better.
    fn update_as_circle_from_radius(
        self,
        radius: impl Into<Scalar>,
        objects: &Objects,
    ) -> Result<Self, ValidationError>;

    /// Update partial half-edge as a line segment, from the given points
    fn update_as_line_segment_from_points(
        self,
        surface: Handle<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self;

    /// Update partial half-edge as a line segment, reusing existing vertices
    fn update_as_line_segment(self) -> Self;

    /// Infer the global form of the partial half-edge
    fn infer_global_form(self) -> Self;
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn with_back_vertex(self, back: impl Into<MaybePartial<Vertex>>) -> Self {
        let [_, front] = self.vertices();
        self.with_vertices([back.into(), front])
    }

    fn with_front_vertex(self, front: impl Into<MaybePartial<Vertex>>) -> Self {
        let [back, _] = self.vertices();
        self.with_vertices([back, front.into()])
    }

    fn update_as_circle_from_radius(
        self,
        radius: impl Into<Scalar>,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let mut curve = self.curve().into_partial();
        curve.global_form = Some(self.extract_global_curve());
        curve.update_as_circle_from_radius(radius);

        let path = curve.path.expect("Expected path that was just created");

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let global_vertex = self
            .global_form()
            .vertices()
            .map(|[global_form, _]| global_form)
            .unwrap_or_else(|| {
                let mut global_vertex = GlobalVertex::partial();
                global_vertex
                    .update_from_curve_and_position(curve.clone(), a_curve);
                global_vertex.into()
            });

        let surface_vertex = PartialSurfaceVertex {
            position: Some(path.point_from_path_coords(a_curve)),
            surface: curve.surface.clone(),
            ..Default::default()
        }
        .with_global_form(Some(global_vertex))
        .build(objects)?
        .insert(objects)?;

        let [back, front] = [a_curve, b_curve].map(|point_curve| {
            Vertex::partial()
                .with_position(Some(point_curve))
                .with_curve(curve.clone())
                .with_surface_form(surface_vertex.clone())
        });

        Ok(self.with_curve(curve).with_vertices([back, front]))
    }

    fn update_as_line_segment_from_points(
        self,
        surface: Handle<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let vertices = points.map(|point| {
            let surface_form = PartialSurfaceVertex {
                position: Some(point.into()),
                surface: Some(surface.clone()),
                ..Default::default()
            };

            Vertex::partial().with_surface_form(surface_form)
        });

        self.with_surface(surface)
            .with_vertices(vertices)
            .update_as_line_segment()
    }

    fn update_as_line_segment(self) -> Self {
        let [from, to] = self.vertices();
        let [from_surface, to_surface] =
            [&from, &to].map(|vertex| vertex.surface_form());

        let surface = self
            .curve()
            .surface()
            .or_else(|| from_surface.surface())
            .or_else(|| to_surface.surface())
            .expect("Can't infer line segment without a surface");
        let points = [&from_surface, &to_surface].map(|vertex| {
            vertex
                .position()
                .expect("Can't infer line segment without surface position")
        });

        let mut curve = PartialCurve {
            surface: Some(surface),
            global_form: Some(self.extract_global_curve()),
            ..Default::default()
        };
        curve.update_as_line_from_points(points);

        let [back, front] = {
            let vertices = [(from, 0.), (to, 1.)].map(|(vertex, position)| {
                vertex.update_partial(|vertex| {
                    vertex
                        .with_position(Some([position]))
                        .with_curve(curve.clone())
                })
            });

            // The global vertices we extracted are in normalized order, which
            // means we might need to switch their order here. This is a bit of
            // a hack, but I can't think of something better.
            let global_forms = {
                let must_switch_order = {
                    let objects = Objects::new();
                    let vertices = vertices.clone().map(|vertex| {
                        vertex
                            .into_full(&objects)
                            .unwrap()
                            .global_form()
                            .clone()
                    });

                    let (_, must_switch_order) =
                        VerticesInNormalizedOrder::new(vertices);

                    must_switch_order
                };

                self.global_form()
                    .vertices()
                    .map(
                        |[a, b]| {
                            if must_switch_order {
                                [b, a]
                            } else {
                                [a, b]
                            }
                        },
                    )
                    .map(|[a, b]| [Some(a), Some(b)])
                    .unwrap_or([None, None])
            };

            vertices
                .into_iter_fixed()
                .zip(global_forms)
                .collect::<[_; 2]>()
                .map(|(vertex, global_form)| {
                    vertex.update_partial(|vertex| {
                        vertex.clone().with_surface_form(
                            vertex.surface_form().update_partial(
                                |surface_vertex| {
                                    surface_vertex.with_global_form(global_form)
                                },
                            ),
                        )
                    })
                })
        };

        self.with_curve(curve).with_vertices([back, front])
    }

    fn infer_global_form(self) -> Self {
        self.with_global_form(PartialGlobalEdge::default())
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    /// Update partial global edge from the given curve and vertices
    fn update_from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self;
}

impl GlobalEdgeBuilder for PartialGlobalEdge {
    fn update_from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self {
        self.with_curve(Some(curve.global_form().clone()))
            .with_vertices(Some(
                vertices.clone().map(|vertex| vertex.global_form().clone()),
            ))
    }
}
