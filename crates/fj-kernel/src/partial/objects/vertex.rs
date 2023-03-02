use fj_math::Point;

use crate::{
    objects::{GlobalVertex, Objects},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`GlobalVertex`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialObject for PartialGlobalVertex {
    type Full = GlobalVertex;

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
            .expect("Can't build `GlobalVertex` without position");

        GlobalVertex::new(position)
    }
}
