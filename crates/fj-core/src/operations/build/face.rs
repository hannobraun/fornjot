use std::{array, borrow::Borrow};

use fj_math::{Point, Scalar};

use crate::{
    Core,
    operations::{
        build::{BuildCycle, BuildRegion, BuildSurface},
        insert::{Insert, IsInserted, IsInsertedNo},
    },
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Surface, Vertex},
};

/// Build a [`Face`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildFace {
    /// Build a face with an empty exterior, no interiors, and no color
    fn unbound(surface: Handle<Surface>, core: &mut Core) -> Face {
        let exterior = Cycle::empty().insert(core);
        let region = Region::new(exterior, []).insert(core);
        Face::new(surface, region)
    }

    /// Build a circle
    fn circle(
        surface: Handle<Surface>,
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Face {
        let region =
            Region::circle(center, radius, surface.clone(), core).insert(core);
        Face::new(surface, region)
    }

    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        core: &mut Core,
    ) -> Polygon<3> {
        let (surface, points_surface) =
            Surface::plane_from_points(points, core);

        let face = Face::polygon(surface, points_surface, core);

        let half_edges = {
            let mut edges =
                face.region().exterior().half_edges().iter().cloned();

            let array = array::from_fn(|_| edges.next()).map(|edge| {
                edge.expect("Just asserted that there are three edges")
            });

            assert!(edges.next().is_none());

            array
        };
        let vertices = half_edges
            .each_ref()
            .map(|edge: &Handle<HalfEdge>| edge.start_vertex().clone());

        Polygon {
            face,
            half_edges,
            vertices,
        }
    }

    /// Build a polygon
    fn polygon<P, Ps>(
        surface: Handle<Surface>,
        points: Ps,
        core: &mut Core,
    ) -> Face
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let region =
            Region::polygon(points, surface.clone(), core).insert(core);
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

    /// The half-edges of the polygon
    pub half_edges: [Handle<HalfEdge>; D],

    /// The vertices of the polygon
    pub vertices: [Handle<Vertex>; D],
}

impl<const D: usize, I: IsInserted> Polygon<D, I> {
    /// Replace the face of the polygon
    ///
    /// Returns a new instance of `Polygon` with the replaced face. Also updates
    /// the other fields of `Polygon` to match the new face.
    pub fn replace_face(&self, face: I::T<Face>) -> Self {
        let half_edges = array::from_fn(|i| {
            face.borrow()
                .region()
                .exterior()
                .half_edges()
                .nth(i)
                .expect("Operation should not have changed length of cycle")
                .clone()
        });
        let vertices = array::from_fn(|i| {
            // The duplicated code here is unfortunate, but unless we get a
            // stable `array::each_ref` and something like `array::unzip`, I'm
            // not sure how to avoid it.
            face.borrow()
                .region()
                .exterior()
                .half_edges()
                .nth(i)
                .expect("Operation should not have changed length of cycle")
                .start_vertex()
                .clone()
        });

        Self {
            face,
            half_edges,
            vertices,
        }
    }
}
