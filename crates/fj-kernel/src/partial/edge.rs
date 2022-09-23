use fj_math::{Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface, Vertex,
    },
    stores::{Handle, Stores},
};

/// API for building a [`HalfEdge`]
///
/// Also see [`HalfEdge::partial`].
pub struct PartialHalfEdge<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The curve that the [`HalfEdge`] is defined in
    pub curve: Option<Curve>,

    /// The vertices that bound this [`HalfEdge`] in the [`Curve`]
    pub vertices: [Option<Vertex>; 2],

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
        self.vertices = vertices.map(Some);
        self
    }

    /// Update the partial half-edge, starting it from the given vertex
    pub fn with_from_vertex(mut self, vertex: Vertex) -> Self {
        self.vertices[0] = Some(vertex);
        self
    }

    /// Update the partial half-edge with the given end vertex
    pub fn with_to_vertex(mut self, vertex: Vertex) -> Self {
        self.vertices[1] = Some(vertex);
        self
    }

    /// Build the [`HalfEdge`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalEdge) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Build the [`HalfEdge`] as a circle from the given radius
    pub fn as_circle_from_radius(
        mut self,
        surface: Surface,
        radius: impl Into<Scalar>,
    ) -> Self {
        let curve = Curve::partial()
            .with_surface(surface)
            .as_circle_from_radius(radius)
            .build(self.stores);

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::partial()
                .from_curve_and_position(curve.clone(), a_curve);

            [a_curve, b_curve].map(|point_curve| {
                Vertex::partial()
                    .with_position(point_curve)
                    .with_curve(curve.clone())
                    .with_global_form(global_vertex.clone())
                    .build(self.stores)
            })
        };

        self.curve = Some(curve);
        self.vertices = vertices.map(Some);

        self
    }

    /// Build the [`HalfEdge`] as a line segment from the given points
    pub fn as_line_segment_from_points(
        mut self,
        surface: Surface,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let curve = Curve::partial()
            .with_surface(surface)
            .as_line_from_points(points)
            .build(self.stores);

        let vertices = [0., 1.].map(|position| {
            Vertex::partial()
                .with_position([position])
                .with_curve(curve.clone())
                .build(self.stores)
        });

        self.curve = Some(curve);
        self.vertices = vertices.map(Some);

        self
    }

    /// Finish building the [`HalfEdge`]
    pub fn build(self) -> HalfEdge {
        let curve = self.curve.expect("Can't build `HalfEdge` without curve");
        let vertices = self.vertices.map(|vertex| {
            vertex.expect("Can't build `HalfEdge` without vertices")
        });

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
