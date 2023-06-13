use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Surface, Vertex},
    operations::{
        BuildCycle, BuildHalfEdge, BuildSurface, Insert, IsInserted,
        IsInsertedNo,
    },
    services::Services,
    storage::Handle,
};

/// Build a [`Face`]
pub trait BuildFace {
    /// Build a face with an empty exterior, no interiors, and no color
    fn unbound(surface: Handle<Surface>, services: &mut Services) -> Face {
        let exterior = Cycle::empty().insert(services);
        let region = Region::new(exterior, [], None);
        Face::new(surface, region)
    }

    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        services: &mut Services,
    ) -> Polygon<3> {
        let [a, b, c] = points.map(Into::into);

        let surface = Surface::plane_from_points([a, b, c]).insert(services);
        let (exterior, edges, vertices) = {
            let half_edges = [[a, b], [b, c], [c, a]].map(|points| {
                let half_edge = HalfEdge::line_segment_from_global_points(
                    points, &surface, None, services,
                );

                half_edge.insert(services)
            });
            let vertices = half_edges
                .each_ref_ext()
                .map(|half_edge| half_edge.start_vertex().clone());

            let cycle = Cycle::new(half_edges.clone()).insert(services);

            (cycle, half_edges, vertices)
        };

        let region = Region::new(exterior, [], None);
        let face = Face::new(surface, region);

        Polygon {
            face,
            edges,
            vertices,
        }
    }
}

impl BuildFace for Face {}

/// A polygon
///
/// # Implementation Note
///
/// Currently code that deals with `Polygon` might assume that the polygon has
/// no holes. Unless you create a `Polygon` yourself, or modify a `Polygon`'s
/// `face` field to have interior cycles, this should not affect you.
pub struct Polygon<const D: usize, I: IsInserted = IsInsertedNo> {
    /// The face that forms the polygon
    pub face: I::T<Face>,

    /// The edges of the polygon
    pub edges: [Handle<HalfEdge>; D],

    /// The vertices of the polygon
    pub vertices: [Handle<Vertex>; D],
}
