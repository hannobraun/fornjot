use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalVertex, HalfEdge, Surface, SurfaceVertex,
        Vertex,
    },
    path::{GlobalPath, SurfacePath},
};

/// API for building an [`HalfEdge`]
pub struct HalfEdgeBuilder {
    surface: Surface,
}

impl HalfEdgeBuilder {
    /// Construct a new instance of [`HalfEdgeBuilder`]
    ///
    /// Also see [`HalfEdge::build`].
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(&self, radius: Scalar) -> HalfEdge {
        let curve = {
            let path = SurfacePath::Circle(Circle::new(
                Point::origin(),
                Vector::from([radius, Scalar::ZERO]),
                Vector::from([Scalar::ZERO, radius]),
            ));
            let global =
                GlobalCurve::from_path(GlobalPath::Circle(Circle::new(
                    Point::origin(),
                    Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
                    Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
                )));

            Curve::new(self.surface, path, global)
        };

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::from_position(
                curve.global_form().path().point_from_path_coords(a_curve),
            );

            let surface_vertices = [a_curve, b_curve].map(|point_curve| {
                let point_surface =
                    curve.path().point_from_path_coords(point_curve);
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

        HalfEdge::from_curve_and_vertices(curve, vertices)
    }

    /// Build a line segment from two points
    pub fn line_segment_from_points(
        &self,
        points: [impl Into<Point<2>>; 2],
    ) -> HalfEdge {
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
            let path = SurfacePath::Line(Line::from_points(points));
            let curve_global = {
                let points = global_vertices
                    .map(|global_vertex| global_vertex.position());
                GlobalCurve::from_path(GlobalPath::Line(Line::from_points(
                    points,
                )))
            };

            Curve::new(self.surface, path, curve_global)
        };

        let vertices = {
            let [a_global, b_global] = global_vertices;
            let [a_surface, b_surface] = surface_vertices;

            [
                Vertex::new(Point::from([0.]), curve, a_surface, a_global),
                Vertex::new(Point::from([1.]), curve, b_surface, b_global),
            ]
        };

        HalfEdge::from_curve_and_vertices(curve, vertices)
    }
}
