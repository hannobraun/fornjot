use fj_math::Point;

use crate::{
    objects::{GlobalEdge, GlobalVertex, HalfEdge, Objects},
    partial::{FullToPartialCache, MaybeCurve, Partial, PartialObject},
    services::Service,
};

/// A partial [`HalfEdge`]
#[derive(Clone, Debug)]
pub struct PartialHalfEdge {
    /// The curve that the half-edge is defined in
    pub curve: Option<MaybeCurve>,

    /// The boundary of the half-edge on the curve
    pub boundary: [Option<Point<1>>; 2],

    /// The surface vertex where the half-edge starts
    pub start_vertex: Partial<GlobalVertex>,

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Compute the surface position where the half-edge starts
    pub fn start_position(&self) -> Option<Point<2>> {
        // Computing the surface position from the curve position is fine.
        // `HalfEdge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary;
        start.and_then(|start| {
            let curve = self.curve?;

            if let MaybeCurve::Defined(curve) = curve {
                return Some(curve.point_from_path_coords(start));
            }

            None
        })
    }
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;

    fn from_full(
        half_edge: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            curve: Some(half_edge.curve().into()),
            boundary: half_edge.boundary().map(Some),
            start_vertex: Partial::from_full(
                half_edge.start_vertex().clone(),
                cache,
            ),
            global_form: Partial::from_full(
                half_edge.global_form().clone(),
                cache,
            ),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let curve = match self.curve.expect("Need path to build curve") {
            MaybeCurve::Defined(path) => path,
            undefined => {
                panic!(
                    "Trying to build curve with undefined path: {undefined:?}"
                )
            }
        };
        let boundary = self.boundary.map(|point| {
            point.expect("Can't build `HalfEdge` without boundary positions")
        });
        let start_vertex = self.start_vertex.build(objects);
        let global_form = self.global_form.build(objects);

        HalfEdge::new(curve, boundary, start_vertex, global_form)
    }
}

impl Default for PartialHalfEdge {
    fn default() -> Self {
        let curve = None;
        let start_vertex = Partial::default();
        let end_vertex = Partial::default();

        let global_form = Partial::from_partial(PartialGlobalEdge {
            vertices: [start_vertex.clone(), end_vertex],
        });

        Self {
            curve,
            boundary: [None; 2],
            start_vertex,
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
