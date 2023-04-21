use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Cycle, Face, HalfEdge, Objects, Surface, Vertex},
    operations::Insert,
    services::Service,
    storage::Handle,
};

use super::{BuildHalfEdge, BuildSurface};

/// Build a [`Face`]
pub trait BuildFace {
    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        objects: &mut Service<Objects>,
    ) -> Triangle {
        let [a, b, c] = points.map(Into::into);

        let surface = Surface::plane_from_points([a, b, c]).insert(objects);
        let (exterior, edges, vertices) = {
            let half_edges = [[a, b], [b, c], [c, a]].map(|points| {
                let half_edge = HalfEdge::line_segment_from_global_points(
                    points, &surface, None, objects,
                );

                half_edge.insert(objects)
            });
            let vertices = half_edges
                .each_ref_ext()
                .map(|half_edge| half_edge.start_vertex().clone());

            let cycle = Cycle::new(half_edges.clone()).insert(objects);

            (cycle, half_edges, vertices)
        };

        let face = Face::new(surface, exterior, [], None);

        Triangle {
            face,
            edges,
            vertices,
        }
    }
}

impl BuildFace for Face {}

/// A triangle
///
/// Returned by [`BuildFace::triangle`].
pub struct Triangle {
    /// The face that forms the triangle
    pub face: Face,

    /// The edges of the triangle
    pub edges: [Handle<HalfEdge>; 3],

    /// The vertices of the triangle
    pub vertices: [Handle<Vertex>; 3],
}
