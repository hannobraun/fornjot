use crate::{
    math::{Plane, Point},
    topology::{face::Face, vertex::Vertex},
};

use super::Handle;

pub struct Sketch {
    pub points: Vec<Point<2>>,
}

impl Sketch {
    pub fn to_face(&self, surface: Handle<Plane>) -> Face {
        let vertices = self
            .points
            .iter()
            .copied()
            .map(|point| {
                let point = surface.point_from_local(point);
                let vertex = Vertex::from(point);
                Handle::new(vertex)
            })
            .collect::<Vec<_>>();

        Face::new(surface, vertices)
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
