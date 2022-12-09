use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    objects::Surface,
    partial::{Partial, PartialGlobalEdge, PartialHalfEdge},
};

use super::{CurveBuilder, SurfaceVertexBuilder};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder: Sized {
    /// Update partial half-edge as a circle, from the given radius
    fn update_as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self;

    /// Update partial half-edge as a line segment, from the given points
    fn update_as_line_segment_from_points(
        self,
        surface: Partial<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self;

    /// Update partial half-edge as a line segment, reusing existing vertices
    fn update_as_line_segment(self) -> Self;
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn update_as_circle_from_radius(
        mut self,
        radius: impl Into<Scalar>,
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

        let global_vertex = surface_vertex.read().global_form.clone();
        self.global_form.write().vertices =
            [global_vertex.clone(), global_vertex];

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

        self.global_form.write().curve = curve.read().global_form.clone();

        self
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalEdgeBuilder for PartialGlobalEdge {}
