use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    objects::{Curve, Objects, Surface, Vertex},
    partial::{PartialGlobalEdge, PartialHalfEdge},
    partial2::Partial,
    services::Service,
    storage::Handle,
};

use super::{CurveBuilder, SurfaceVertexBuilder};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder: Sized {
    /// Update the partial half-edge with the given back vertex
    fn with_back_vertex(self, back: Partial<Vertex>) -> Self;

    /// Update the partial half-edge with the given front vertex
    fn with_front_vertex(self, front: Partial<Vertex>) -> Self;

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
        objects: &mut Service<Objects>,
    ) -> Self;

    /// Update partial half-edge as a line segment, from the given points
    fn update_as_line_segment_from_points(
        self,
        surface: Partial<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self;

    /// Update partial half-edge as a line segment, reusing existing vertices
    fn update_as_line_segment(self) -> Self;

    /// Infer the global form of the partial half-edge
    fn infer_global_form(self) -> Self;
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn with_back_vertex(mut self, back: Partial<Vertex>) -> Self {
        let [_, front] = self.vertices.clone();
        self.vertices = [back, front];
        self
    }

    fn with_front_vertex(mut self, front: Partial<Vertex>) -> Self {
        let [back, _] = self.vertices.clone();
        self.vertices = [back, front];
        self
    }

    fn update_as_circle_from_radius(
        mut self,
        radius: impl Into<Scalar>,
        _: &mut Service<Objects>,
    ) -> Self {
        let mut curve = self.curve();
        curve.write().update_as_circle_from_radius(radius);

        let path = curve
            .read()
            .path
            .expect("Expected path that was just created");

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let [vertex, _] = &mut self.vertices;

        let mut surface_vertex = vertex.write().surface_form.clone();
        surface_vertex.write().position =
            Some(path.point_from_path_coords(a_curve));

        for (vertex, point_curve) in
            self.vertices.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            let mut vertex = vertex.write();
            vertex.position = Some(point_curve);
            vertex.surface_form = surface_vertex.clone();
        }

        self
    }

    fn update_as_line_segment_from_points(
        mut self,
        surface: Partial<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        for (vertex, point) in self.vertices.each_mut_ext().zip_ext(points) {
            let mut vertex = vertex.write();

            vertex.curve.write().surface = surface.clone();

            let mut surface_form = vertex.surface_form.write();
            surface_form.position = Some(point.into());
            surface_form.surface = surface.clone();
            surface_form.infer_global_position();
        }

        self.update_as_line_segment()
    }

    fn update_as_line_segment(mut self) -> Self {
        let [from, to] = self.vertices.clone();
        let [from_surface, to_surface] =
            [&from, &to].map(|vertex| vertex.read().surface_form.clone());

        let surface = self.curve().read().surface.clone();
        let points = [&from_surface, &to_surface].map(|vertex| {
            vertex
                .read()
                .position
                .expect("Can't infer line segment without surface position")
        });

        let mut curve = self.curve();
        curve.write().surface = surface;
        curve.write().update_as_line_from_points(points);

        let [back, front] = {
            [(from, 0.), (to, 1.)].map(|(mut vertex, position)| {
                vertex.write().position = Some([position].into());
                vertex.write().curve = self.curve();
                vertex
            })
        };

        self.vertices = [back, front];

        self
    }

    fn infer_global_form(mut self) -> Self {
        self.global_form = PartialGlobalEdge::default().into();
        self
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
        mut self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self {
        self.curve =
            Partial::from_full_entry_point(curve.global_form().clone());
        self.vertices = vertices.clone().map(|vertex| {
            Partial::from_full_entry_point(vertex.global_form().clone())
        });
        self
    }
}
