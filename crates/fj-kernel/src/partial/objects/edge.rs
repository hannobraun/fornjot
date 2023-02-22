use std::array;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalEdge, GlobalVertex, HalfEdge, Objects, SurfaceVertex,
    },
    partial::{FullToPartialCache, Partial, PartialObject},
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
        let vertices = self.vertices.map(|vertex| {
            let position_curve = vertex
                .0
                .expect("Can't build `HalfEdge` without boundary positions");
            let surface_form = vertex.1.build(objects);

            (position_curve, surface_form)
        });
        let global_form = self.global_form.build(objects);

        HalfEdge::new(curve, vertices, global_form)
    }
}

impl Default for PartialHalfEdge {
    fn default() -> Self {
        let curve = Partial::<Curve>::new();
        let vertices = array::from_fn(|_| {
            let surface_form = Partial::default();
            (None, surface_form)
        });

        let global_vertices = vertices.each_ref_ext().map(
            |vertex: &(Option<Point<1>>, Partial<SurfaceVertex>)| {
                let surface_vertex = vertex.1.clone();
                let global_vertex = surface_vertex.read().global_form.clone();
                global_vertex
            },
        );

        let global_form = Partial::from_partial(PartialGlobalEdge {
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
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(|vertex| Partial::from_full(vertex, cache)),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let vertices = self.vertices.map(|vertex| vertex.build(objects));
        GlobalEdge::new(vertices)
    }
}
