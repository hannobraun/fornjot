use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Surface, Vertex},
    operations::{
        BuildCycle, BuildRegion, BuildSurface, Insert, IsInserted, IsInsertedNo,
    },
    services::Services,
    storage::Handle,
};

/// Build a [`Face`]
pub trait BuildFace {
    /// Build a face with an empty exterior, no interiors, and no color
    fn unbound(surface: Handle<Surface>, services: &mut Services) -> Face {
        let exterior = Cycle::empty().insert(services);
        let region = Region::new(exterior, [], None).insert(services);
        Face::new(surface, region)
    }

    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        services: &mut Services,
    ) -> Polygon<3> {
        let (surface, points_surface) = Surface::plane_from_points(points);
        let surface = surface.insert(services);

        let face = Face::polygon(surface, points_surface, services);

        let edges = {
            let mut half_edges = face.region().exterior().half_edges().cloned();
            assert_eq!(half_edges.clone().count(), 3);

            [half_edges.next(), half_edges.next(), half_edges.next()].map(
                |half_edge| {
                    half_edge
                        .expect("Just asserted that there are three half-edges")
                },
            )
        };
        let vertices = edges
            .each_ref_ext()
            .map(|half_edge| half_edge.start_vertex().clone());

        Polygon {
            face,
            edges,
            vertices,
        }
    }

    /// Build a polygon
    fn polygon<P, Ps>(
        surface: Handle<Surface>,
        points: Ps,
        services: &mut Services,
    ) -> Face
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let region = Region::polygon(points, services).insert(services);
        Face::new(surface, region)
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
