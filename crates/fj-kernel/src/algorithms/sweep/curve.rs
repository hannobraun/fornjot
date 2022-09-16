use fj_math::Vector;

use crate::{
    objects::{Curve, GlobalCurve, Surface},
    stores::Stores,
};

use super::Sweep;

impl Sweep for Curve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<Vector<3>>, stores: &Stores) -> Self::Swept {
        self.global_form().sweep(path, stores)
    }
}

impl Sweep for GlobalCurve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<Vector<3>>, _: &Stores) -> Self::Swept {
        Surface::new(self.path(), path.into())
    }
}
