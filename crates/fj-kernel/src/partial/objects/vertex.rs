use fj_math::Point;

use crate::{
    objects::{Objects, Vertex},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`Vertex`]
#[derive(Clone, Debug)]
pub struct PartialVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialObject for PartialVertex {
    type Full = Vertex;

    fn new(_: &mut Service<Objects>) -> Self {
        Self { position: None }
    }

    fn from_full(
        global_vertex: &Self::Full,
        _: &mut FullToPartialCache,
    ) -> Self {
        Self {
            position: Some(global_vertex.position()),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `Vertex` without position");

        Vertex::new(position)
    }
}
