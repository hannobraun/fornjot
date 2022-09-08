use crate::objects::{Curve, GlobalCurve, Surface, SweptCurve};

use super::Sweep;

impl Sweep for Curve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<super::Path>) -> Self::Swept {
        self.global_form().sweep(path)
    }
}

impl Sweep for GlobalCurve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<super::Path>) -> Self::Swept {
        Surface::SweptCurve(SweptCurve {
            curve: *self.kind(),
            path: path.into().inner(),
        })
    }
}
