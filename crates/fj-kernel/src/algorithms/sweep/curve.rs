use fj_math::Vector;

use crate::objects::{Curve, GlobalCurve, Surface};

use super::Sweep;

impl Sweep for Curve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        self.global_form().sweep(path)
    }
}

impl Sweep for GlobalCurve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        Surface::new(*self.path(), path.into())
    }
}
