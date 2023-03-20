use fj_math::Transform;

use crate::{objects::Set, operations::Insert, services::Services};

use super::{TransformCache, TransformObject};

impl<T: Ord + TransformObject + Clone + Insert + 'static> TransformObject
    for Set<T>
{
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let mut inner = Self::new();
        inner.extend(self.into_iter().map(|i| {
            TransformObject::transform_with_cache(i, transform, services, cache)
        }));
        inner
    }
}
