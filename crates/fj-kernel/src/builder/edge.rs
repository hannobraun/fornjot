use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::{
    local::Local,
    objects::{Curve, Edge, GlobalVertex, Surface, Vertex, VerticesOfEdge},
};

/// API for building an [`Edge`]
pub struct EdgeBuilder;

impl EdgeBuilder {
    /// Create a circle from the given radius
    pub fn circle_from_radius(&self, radius: Scalar) -> Edge {
        let curve_local = Curve::Circle(Circle {
            center: Point::origin(),
            a: Vector::from([radius, Scalar::ZERO]),
            b: Vector::from([Scalar::ZERO, radius]),
        });
        let curve_canonical = Curve::Circle(Circle {
            center: Point::origin(),
            a: Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
            b: Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
        });

        Edge::new(
            Local::new(curve_local, curve_canonical),
            VerticesOfEdge::none(),
        )
    }

    /// Create a line segment from two points
    pub fn line_segment_from_points(
        &self,
        surface: &Surface,
        points: [impl Into<Point<2>>; 2],
    ) -> Edge {
        let points = points.map(Into::into);

        let global_vertices = points.map(|position| {
            let position = surface.point_from_surface_coords(position);
            GlobalVertex::from_position(position)
        });

        let curve_local = Curve::Line(Line::from_points(points));
        let curve_canonical = {
            let points =
                global_vertices.map(|global_vertex| global_vertex.position());
            Curve::Line(Line::from_points(points))
        };

        let vertices = {
            let [a, b] = global_vertices;
            [
                Vertex::new(Point::from([0.]), a),
                Vertex::new(Point::from([1.]), b),
            ]
        };

        Edge::new(
            Local::new(curve_local, curve_canonical),
            VerticesOfEdge::from_vertices(vertices),
        )
    }
}
