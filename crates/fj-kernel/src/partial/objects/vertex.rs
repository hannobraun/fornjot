use fj_math::Point;

use crate::{
    objects::{GlobalVertex, Objects, SurfaceVertex},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`SurfaceVertex`]
#[derive(Clone, Debug, Default)]
pub struct PartialSurfaceVertex {
    /// The global form of the vertex
    pub global_form: Partial<GlobalVertex>,
}

impl PartialObject for PartialSurfaceVertex {
    type Full = SurfaceVertex;

    fn from_full(
        surface_vertex: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            global_form: Partial::from_full(
                surface_vertex.global_form().clone(),
                cache,
            ),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let global_form = self.global_form.build(objects);
        SurfaceVertex::new(global_form)
    }
}

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
