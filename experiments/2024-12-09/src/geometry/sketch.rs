use crate::{
    math::{Plane, Point},
    object::Handle,
    topology::{face::Face, half_edge::HalfEdge, vertex::Vertex},
};

pub struct Sketch {
    pub points: Vec<Point<2>>,
}

impl Sketch {
    pub fn to_face(&self, surface: Plane) -> Face {
        let vertices = self
            .points
            .iter()
            .copied()
            .map(|point| {
                let point = surface.point_from_local(point);
                Handle::new(Vertex::new(point))
            })
            .collect::<Vec<_>>();

        let half_edges = vertices
            .into_iter()
            .map(|start| Handle::new(HalfEdge { start }));

        Face::new(surface, half_edges)
    }
}

impl<I, P> From<I> for Sketch
where
    I: IntoIterator<Item = P>,
    P: Into<Point<2>>,
{
    fn from(points: I) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}
