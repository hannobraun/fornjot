use std::array;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        SurfaceVertex,
    },
    partial::{
        FullToPartialCache, Partial, PartialObject, PartialSurfaceVertex,
    },
    services::Service,
};

/// A partial [`HalfEdge`]
#[derive(Clone, Debug)]
pub struct PartialHalfEdge {
    /// The curve that the half-edge is defined in
    pub curve: Partial<Curve>,

    /// The vertices that bound the half-edge on the curve
    pub vertices: [(Option<Point<1>>, Partial<SurfaceVertex>); 2],

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;

    fn from_full(
        half_edge: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            curve: Partial::from_full(half_edge.curve().clone(), cache),
            vertices: half_edge
                .boundary()
                .zip_ext(half_edge.surface_vertices())
                .map(|(position, surface_vertex)| {
                    (
                        Some(position),
                        Partial::from_full(surface_vertex.clone(), cache),
                    )
                }),
            global_form: Partial::from_full(
                half_edge.global_form().clone(),
                cache,
            ),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let curve = self.curve.build(objects);
        let vertices = self.vertices.map(|mut vertex| {
            let position_surface = vertex.1.read().position;

            // Infer surface position, if not available.
            let position_surface = match position_surface {
                Some(position_surface) => position_surface,
                None => {
                    let position_curve = vertex.0.expect(
                        "Can't infer surface position without curve position",
                    );
                    let position_surface =
                        curve.path().point_from_path_coords(position_curve);

                    vertex.1.write().position = Some(position_surface);

                    position_surface
                }
            };

            // Infer global position, if not available.
            let position_global = vertex.1.read().global_form.read().position;
            if position_global.is_none() {
                let surface = curve.surface().geometry();

                let position_global =
                    surface.point_from_surface_coords(position_surface);
                vertex.1.write().global_form.write().position =
                    Some(position_global);
            }

            let position =
                vertex.0.expect("Can't build `Vertex` without position");
            let surface_form = vertex.1.build(objects);

            (position, surface_form)
        });
        let global_form = self.global_form.build(objects);

        HalfEdge::new(curve, vertices, global_form)
    }
}

impl Default for PartialHalfEdge {
    fn default() -> Self {
        let curve = Partial::<Curve>::new();
        let vertices = array::from_fn(|_| {
            let surface = Partial::new();

            let surface_form = Partial::from_partial(PartialSurfaceVertex {
                surface,
                ..Default::default()
            });

            (None, surface_form)
        });

        let global_curve = curve.read().global_form.clone();
        let global_vertices = vertices.each_ref_ext().map(
            |vertex: &(Option<Point<1>>, Partial<SurfaceVertex>)| {
                let surface_vertex = vertex.1.clone();
                let global_vertex = surface_vertex.read().global_form.clone();
                global_vertex
            },
        );

        let global_form = Partial::from_partial(PartialGlobalEdge {
            curve: global_curve,
            vertices: global_vertices,
        });

        Self {
            curve,
            vertices,
            global_form,
        }
    }
}

/// A partial [`GlobalEdge`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    /// The curve that defines the edge's geometry
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the edge on the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}

impl PartialObject for PartialGlobalEdge {
    type Full = GlobalEdge;

    fn from_full(
        global_edge: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            curve: Partial::from_full(global_edge.curve().clone(), cache),
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(|vertex| Partial::from_full(vertex, cache)),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let curve = self.curve.build(objects);
        let vertices = self.vertices.map(|vertex| vertex.build(objects));

        GlobalEdge::new(curve, vertices)
    }
}
