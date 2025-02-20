use fj_math::Transform;

use crate::{Core, topology::Vertex};

use super::{TransformCache, TransformObject};

impl TransformObject for Vertex {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Core,
        _: &mut TransformCache,
    ) -> Self::Transformed {
        // There's nothing to actually transform here, as `Vertex` holds no
        // data. We still need this implementation though, as a new `Vertex`
        // object must be created to represent the new and transformed vertex.
        Self::new()
    }
}
