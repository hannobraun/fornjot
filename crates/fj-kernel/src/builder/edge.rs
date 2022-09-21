use fj_math::{Line, Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface,
        SurfaceVertex, Vertex,
    },
    path::{GlobalPath, SurfacePath},
    stores::Stores,
};

/// API for building an [`HalfEdge`]
///
/// Also see [`HalfEdge::builder`].
pub struct HalfEdgeBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`HalfEdge`] is defined in
    pub surface: Surface,
}

impl<'a> HalfEdgeBuilder<'a> {
    /// Build a circle from the given radius
    pub fn build_circle_from_radius(
        self,
        radius: impl Into<Scalar>,
    ) -> HalfEdge {
        let curve = Curve::builder(self.stores, self.surface)
            .build_circle_from_radius(radius);

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::builder()
                .build_from_curve_and_position(&curve, a_curve);

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
                        curve.clone(),
                        surface_vertex,
                        global_vertex,
                    )
                },
            )
        };

        HalfEdge::from_curve_and_vertices(curve, vertices)
    }

    /// Build a line segment from two points
    pub fn build_line_segment_from_points(
        self,
        points: [impl Into<Point<2>>; 2],
    ) -> HalfEdge {
        let points = points.map(Into::into);

        let global_vertices = points.map(|position| {
            GlobalVertex::builder()
                .build_from_surface_and_position(&self.surface, position)
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
            let global_form = {
                let points = global_vertices
                    .map(|global_vertex| global_vertex.position());
                self.stores.global_curves.insert(GlobalCurve::from_path(
                    GlobalPath::Line(Line::from_points(points)),
                ))
            };

            Curve::new(self.surface, path, global_form)
        };

        let vertices = {
            let [a_global, b_global] = global_vertices;
            let [a_surface, b_surface] = surface_vertices;

            [
                Vertex::new(
                    Point::from([0.]),
                    curve.clone(),
                    a_surface,
                    a_global,
                ),
                Vertex::new(
                    Point::from([1.]),
                    curve.clone(),
                    b_surface,
                    b_global,
                ),
            ]
        };

        HalfEdge::from_curve_and_vertices(curve, vertices)
    }
}

/// API for building a [`GlobalEdge`]
///
/// Also see [`GlobalEdge::builder`].
pub struct GlobalEdgeBuilder;

impl GlobalEdgeBuilder {
    /// Build a [`GlobalEdge`] from the provided curve and vertices
    pub fn build_from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Vertex; 2],
    ) -> GlobalEdge {
        GlobalEdge::new(
            curve.global_form().clone(),
            vertices.clone().map(|vertex| *vertex.global_form()),
        )
    }
}
