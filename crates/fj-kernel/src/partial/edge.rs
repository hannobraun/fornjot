use fj_math::{Line, Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface,
        SurfaceVertex, Vertex,
    },
    path::{GlobalPath, SurfacePath},
    stores::{Handle, Stores},
};

/// API for building a [`HalfEdge`]
///
/// Also see [`HalfEdge::builder`].
pub struct PartialHalfEdge<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`HalfEdge`]'s [`Curve`] is defined in
    pub surface: Surface,

    /// The curve that the [`HalfEdge`] is defined in
    pub curve: Option<Curve>,

    /// The vertices that bound this [`HalfEdge`] in the [`Curve`]
    pub vertices: Option<[Vertex; 2]>,

    /// The global form of the [`HalfEdge`]
    ///
    /// Can be provided to the builder, if available, or computed by one of the
    /// build methods.
    pub global_form: Option<GlobalEdge>,
}

impl<'a> PartialHalfEdge<'a> {
    /// Build the [`HalfEdge`] with the given curve
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = Some(curve);
        self
    }

    /// Build the [`HalfEdge`] with the given vertices
    pub fn with_vertices(mut self, vertices: [Vertex; 2]) -> Self {
        self.vertices = Some(vertices);
        self
    }

    /// Build the [`HalfEdge`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalEdge) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Build the [`HalfEdge`] as a circle from the given radius
    pub fn as_circle_from_radius(mut self, radius: impl Into<Scalar>) -> Self {
        let curve = Curve::partial()
            .with_surface(self.surface)
            .as_circle_from_radius(radius)
            .build(self.stores);

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::partial()
                .from_curve_and_position(curve.clone(), a_curve)
                .build(self.stores);

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

        self.curve = Some(curve);
        self.vertices = Some(vertices);

        self
    }

    /// Build the [`HalfEdge`] as a line segment from the given points
    pub fn as_line_segment_from_points(
        mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let points = points.map(Into::into);

        let global_vertices = points.map(|position| {
            GlobalVertex::partial()
                .from_surface_and_position(&self.surface, position)
                .build(self.stores)
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

        self.curve = Some(curve);
        self.vertices = Some(vertices);

        self
    }

    /// Finish building the [`HalfEdge`]
    pub fn build(self) -> HalfEdge {
        let curve = self.curve.expect("Can't build `HalfEdge` without curve");
        let vertices = self
            .vertices
            .expect("Can't build `HalfEdge` without vertices");

        let global_form = self.global_form.unwrap_or_else(|| {
            GlobalEdge::partial()
                .from_curve_and_vertices(&curve, &vertices)
                .build(self.stores)
        });

        HalfEdge::new(curve, vertices, global_form)
    }
}

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalEdge {
    /// The curve that the [`GlobalEdge`] is defined in
    ///
    /// Must be provided before [`PartialGlobalEdge::build`] is called.
    pub curve: Option<Handle<GlobalCurve>>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    ///
    /// Must be provided before [`PartialGlobalEdge::build`] is called.
    pub vertices: Option<[GlobalVertex; 2]>,
}

impl PartialGlobalEdge {
    /// Update partial global edge from the given curve and vertices
    pub fn from_curve_and_vertices(
        mut self,
        curve: &Curve,
        vertices: &[Vertex; 2],
    ) -> Self {
        self.curve = Some(curve.global_form().clone());
        self.vertices =
            Some(vertices.clone().map(|vertex| *vertex.global_form()));

        self
    }

    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(self, _: &Stores) -> GlobalEdge {
        let curve = self
            .curve
            .expect("Can't build `GlobalEdge` without `GlobalCurve`");
        let vertices = self
            .vertices
            .expect("Can't build `GlobalEdge` without vertices");

        GlobalEdge::new(curve, vertices)
    }
}

impl From<GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: GlobalEdge) -> Self {
        Self {
            curve: Some(global_edge.curve().clone()),
            vertices: Some(*global_edge.vertices()),
        }
    }
}
