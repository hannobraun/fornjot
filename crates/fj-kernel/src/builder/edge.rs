use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::objects::{
    Curve, CurveKind, Edge, GlobalCurve, GlobalVertex, Surface, SurfaceVertex,
    Vertex,
};

/// API for building an [`Edge`]
pub struct EdgeBuilder {
    surface: Surface,
}

impl EdgeBuilder {
    /// Construct a new instance of [`EdgeBuilder`]
    ///
    /// Also see [`Edge::build`].
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    /// Create a circle from the given radius
    pub fn circle_from_radius(&self, radius: Scalar) -> Edge {
        let curve = {
            let local = CurveKind::Circle(Circle::new(
                Point::origin(),
                Vector::from([radius, Scalar::ZERO]),
                Vector::from([Scalar::ZERO, radius]),
            ));
            let global =
                GlobalCurve::from_kind(CurveKind::Circle(Circle::new(
                    Point::origin(),
                    Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
                    Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
                )));

            Curve::new(self.surface, local, global)
        };

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::from_position(
                curve.global_form().kind().point_from_curve_coords(a_curve),
            );

            let surface_vertices = [a_curve, b_curve].map(|point_curve| {
                let point_surface =
                    curve.kind().point_from_curve_coords(point_curve);
                SurfaceVertex::new(point_surface, self.surface, global_vertex)
            });

            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = surface_vertices;
            [(a_curve, a_surface), (b_curve, b_surface)].map(
                |(point_curve, surface_vertex)| {
                    Vertex::new(
                        point_curve,
                        curve,
                        surface_vertex,
                        global_vertex,
                    )
                },
            )
        };

        Edge::from_curve_and_vertices(curve, vertices)
    }

    /// Create a line segment from two points
    pub fn line_segment_from_points(
        &self,
        points: [impl Into<Point<2>>; 2],
    ) -> Edge {
        let points = points.map(Into::into);

        let global_vertices = points.map(|position| {
            let position = self.surface.point_from_surface_coords(position);
            GlobalVertex::from_position(position)
        });

        let surface_vertices = {
            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = points;
            let [a_global, b_global] = global_vertices;
            [(a_surface, a_global), (b_surface, b_global)].map(
                |(point_surface, vertex_global)| {
                    SurfaceVertex::new(
                        point_surface,
                        self.surface,
                        vertex_global,
                    )
                },
            )
        };

        let curve = {
            let curve_local = CurveKind::Line(Line::from_points(points));
            let curve_global = {
                let points = global_vertices
                    .map(|global_vertex| global_vertex.position());
                let kind = CurveKind::Line(Line::from_points(points));
                GlobalCurve::from_kind(kind)
            };

            Curve::new(self.surface, curve_local, curve_global)
        };

        let vertices = {
            let [a_global, b_global] = global_vertices;
            let [a_surface, b_surface] = surface_vertices;

            [
                Vertex::new(Point::from([0.]), curve, a_surface, a_global),
                Vertex::new(Point::from([1.]), curve, b_surface, b_global),
            ]
        };

        Edge::from_curve_and_vertices(curve, vertices)
    }
}
