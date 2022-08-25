use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::objects::{
    Curve, CurveKind, Edge, GlobalCurve, GlobalVertex, Surface, Vertex,
    VerticesOfEdge,
};

/// API for building an [`Edge`]
pub struct EdgeBuilder;

impl EdgeBuilder {
    /// Create a circle from the given radius
    pub fn circle_from_radius(&self, radius: Scalar) -> Edge {
        let curve_local = CurveKind::Circle(Circle::new(
            Point::origin(),
            Vector::from([radius, Scalar::ZERO]),
            Vector::from([Scalar::ZERO, radius]),
        ));
        let curve_global =
            GlobalCurve::from_kind(CurveKind::Circle(Circle::new(
                Point::origin(),
                Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
                Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
            )));

        Edge::from_curve_and_vertices(
            Curve::new(curve_local, curve_global),
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

        let curve_local = CurveKind::Line(Line::from_points(points));
        let curve_global = {
            let points =
                global_vertices.map(|global_vertex| global_vertex.position());
            let kind = CurveKind::Line(Line::from_points(points));
            GlobalCurve::from_kind(kind)
        };

        let vertices = {
            let [a, b] = global_vertices;
            [
                Vertex::new(Point::from([0.]), a),
                Vertex::new(Point::from([1.]), b),
            ]
        };

        Edge::from_curve_and_vertices(
            Curve::new(curve_local, curve_global),
            VerticesOfEdge::from_vertices(vertices),
        )
    }
}
