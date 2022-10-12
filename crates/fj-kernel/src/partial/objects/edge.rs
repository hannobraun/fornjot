use fj_math::{Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, SurfaceVertex, Vertex,
    },
    partial::{HasPartial, MaybePartial},
    storage::{Handle, HandleWrapper},
};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialHalfEdge {
    /// The surface that the [`HalfEdge`]'s [`Curve`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The curve that the [`HalfEdge`] is defined in
    pub curve: Option<MaybePartial<Handle<Curve>>>,

    /// The vertices that bound this [`HalfEdge`] in the [`Curve`]
    pub vertices: [Option<MaybePartial<Vertex>>; 2],

    /// The global form of the [`HalfEdge`]
    ///
    /// Can be computed by [`PartialHalfEdge::build`], if not available.
    pub global_form: Option<MaybePartial<GlobalEdge>>,
}

impl PartialHalfEdge {
    /// Extract the global curve from either the curve or global form
    ///
    /// If a global curve is available through both, the curve is preferred.
    pub fn extract_global_curve(&self) -> Option<Handle<GlobalCurve>> {
        let global_curve_from_curve = || self.curve.as_ref()?.global_form();
        let global_curve_from_global_form =
            || Some(self.global_form.as_ref()?.curve()?.clone());

        global_curve_from_curve().or_else(global_curve_from_global_form)
    }

    /// Update the partial half-edge with the given surface
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            self.surface = Some(surface);
        }
        self
    }

    /// Update the partial half-edge with the given curve
    pub fn with_curve(
        mut self,
        curve: Option<impl Into<MaybePartial<Handle<Curve>>>>,
    ) -> Self {
        if let Some(curve) = curve {
            self.curve = Some(curve.into());
        }
        self
    }

    /// Update the partial half-edge with the given from vertex
    pub fn with_from_vertex(
        mut self,
        vertex: Option<impl Into<MaybePartial<Vertex>>>,
    ) -> Self {
        if let Some(vertex) = vertex {
            let [from, _] = &mut self.vertices;
            *from = Some(vertex.into());
        }
        self
    }

    /// Update the partial half-edge with the given from vertex
    pub fn with_to_vertex(
        mut self,
        vertex: Option<impl Into<MaybePartial<Vertex>>>,
    ) -> Self {
        if let Some(vertex) = vertex {
            let [_, to] = &mut self.vertices;
            *to = Some(vertex.into());
        }
        self
    }

    /// Update the partial half-edge with the given vertices
    pub fn with_vertices(
        mut self,
        vertices: Option<[impl Into<MaybePartial<Vertex>>; 2]>,
    ) -> Self {
        let vertices = vertices.map(|vertices| vertices.map(Into::into));
        if let Some([a, b]) = vertices {
            self.vertices = [Some(a), Some(b)];
        }
        self
    }

    /// Update the partial half-edge with the given global form
    pub fn with_global_form(
        mut self,
        global_form: Option<impl Into<MaybePartial<GlobalEdge>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = Some(global_form.into());
        }
        self
    }

    /// Update partial half-edge as a circle, from the given radius
    pub fn as_circle_from_radius(mut self, radius: impl Into<Scalar>) -> Self {
        let curve = Handle::<Curve>::partial()
            .with_global_form(self.extract_global_curve())
            .with_surface(self.surface.clone())
            .as_circle_from_radius(radius);

        let [a, b] = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_form = Handle::<GlobalVertex>::partial()
                .from_curve_and_position(curve.clone(), a_curve);

            let path = curve.path.expect("Expected path that was just created");
            let surface_form = SurfaceVertex::partial()
                .with_position(Some(path.point_from_path_coords(a_curve)))
                .with_global_form(Some(global_form));

            [a_curve, b_curve].map(|point_curve| {
                Vertex::partial()
                    .with_position(Some(point_curve))
                    .with_curve(Some(curve.clone()))
                    .with_surface_form(Some(surface_form.clone()))
                    .into()
            })
        };

        self.curve = Some(curve.into());
        self.vertices = [Some(a), Some(b)];

        self
    }

    /// Update partial half-edge as a line segment, from the given points
    pub fn as_line_segment_from_points(
        self,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let surface = self.surface.clone();
        let vertices = points.map(|point| {
            let surface_form = SurfaceVertex::partial()
                .with_surface(surface.clone())
                .with_position(Some(point));
            Vertex::partial().with_surface_form(Some(surface_form))
        });

        self.with_vertices(Some(vertices)).as_line_segment()
    }

    /// Update partial half-edge as a line segment, reusing existing vertices
    pub fn as_line_segment(mut self) -> Self {
        let [from, to] = self.vertices.clone().map(|vertex| {
            vertex.expect("Can't infer line segment without vertices")
        });
        let [from_surface, to_surface] = [&from, &to].map(|vertex| {
            vertex
                .surface_form()
                .expect("Can't infer line segment without two surface vertices")
        });

        let surface = self
            .surface
            .as_ref()
            .or_else(|| from_surface.surface())
            .or_else(|| to_surface.surface())
            .expect("Can't infer line segment without a surface")
            .clone();
        let points = [&from_surface, &to_surface].map(|vertex| {
            vertex
                .position()
                .expect("Can't infer line segment without surface position")
        });

        let curve = Handle::<Curve>::partial()
            .with_global_form(self.extract_global_curve())
            .with_surface(Some(surface))
            .as_line_from_points(points);

        let [a, b] = [(from, 0.), (to, 1.)].map(|(vertex, position)| {
            vertex.update_partial(|vertex| {
                vertex
                    .with_position(Some([position]))
                    .with_curve(Some(curve.clone()))
            })
        });

        self.curve = Some(curve.into());
        self.vertices = [Some(a), Some(b)];

        self
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(self, objects: &Objects) -> HalfEdge {
        let surface = self.surface;
        let curve = self
            .curve
            .expect("Can't build `HalfEdge` without curve")
            .update_partial(|curve| curve.with_surface(surface))
            .into_full(objects);
        let vertices = self.vertices.map(|vertex| {
            vertex
                .expect("Can't build `HalfEdge` without vertices")
                .update_partial(|vertex| vertex.with_curve(Some(curve.clone())))
                .into_full(objects)
        });

        let global_form = self
            .global_form
            .unwrap_or_else(|| GlobalEdge::partial().into())
            .update_partial(|partial| {
                partial.from_curve_and_vertices(&curve, &vertices)
            })
            .into_full(objects);

        HalfEdge::new(vertices, global_form)
    }
}

impl From<&HalfEdge> for PartialHalfEdge {
    fn from(half_edge: &HalfEdge) -> Self {
        let [a, b] = half_edge.vertices().clone().map(Into::into);

        Self {
            surface: Some(half_edge.curve().surface().clone()),
            curve: Some(half_edge.curve().clone().into()),
            vertices: [Some(a), Some(b)],
            global_form: Some(half_edge.global_form().clone().into()),
        }
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
    pub curve: Option<HandleWrapper<GlobalCurve>>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    ///
    /// Must be provided before [`PartialGlobalEdge::build`] is called.
    pub vertices: Option<[Handle<GlobalVertex>; 2]>,
}

impl PartialGlobalEdge {
    /// Update the partial global edge with the given curve
    pub fn with_curve(mut self, curve: Option<Handle<GlobalCurve>>) -> Self {
        if let Some(curve) = curve {
            self.curve = Some(curve.into());
        }
        self
    }

    /// Update the partial global edge with the given vertices
    pub fn with_vertices(
        mut self,
        vertices: Option<[Handle<GlobalVertex>; 2]>,
    ) -> Self {
        if let Some(vertices) = vertices {
            self.vertices = Some(vertices);
        }
        self
    }

    /// Update partial global edge from the given curve and vertices
    pub fn from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Vertex; 2],
    ) -> Self {
        self.with_curve(Some(curve.global_form().clone()))
            .with_vertices(Some(
                vertices.clone().map(|vertex| vertex.global_form().clone()),
            ))
    }

    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(self, _: &Objects) -> GlobalEdge {
        let curve = self
            .curve
            .expect("Can't build `GlobalEdge` without `GlobalCurve`");
        let vertices = self
            .vertices
            .expect("Can't build `GlobalEdge` without vertices");

        GlobalEdge::new(curve, vertices)
    }
}

impl From<&GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: &GlobalEdge) -> Self {
        Self {
            curve: Some(global_edge.curve().clone().into()),
            vertices: Some(
                global_edge.vertices().access_in_normalized_order().clone(),
            ),
        }
    }
}
