pub mod face;

use crate::{
    objects::{Face, Objects},
    services::Service,
};

use super::{HasPartial, MaybePartial, Partial, PartialFace};

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartial for $full {
                type Partial = $partial;
            }

            impl Partial for $partial {
                type Full = $full;

                fn build(self, objects: &mut Service<Objects>) -> Self::Full {
                    self.build(objects)
                }
            }

            impl From<$partial> for MaybePartial<$full> {
                fn from(partial: $partial) -> Self {
                    Self::Partial(partial)
                }
            }
        )*
    };
}

impl_traits!(
    Face, PartialFace;
);
