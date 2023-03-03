use fj_math::Transform;

use crate::{
    objects::{Objects, Vertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Vertex {
    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        // There's nothing to actually transform here, as `Vertex` holds no
        // data. We still need this implementation though, as a new `Vertex`
        // object must be created to represent the new and transformed vertex.
        Self::new()
    }
}
