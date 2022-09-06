use crate::objects::{Curve, GlobalCurve, Surface, SweptCurve};

use super::Sweep;

impl Sweep for Curve {
    type Swept = Surface;

    fn sweep(
        self,
        path: impl Into<super::Path>,
        color: fj_interop::mesh::Color,
    ) -> Self::Swept {
        self.global_form().sweep(path, color)
    }
}

impl Sweep for GlobalCurve {
    type Swept = Surface;

    fn sweep(
        self,
        path: impl Into<super::Path>,
        _: fj_interop::mesh::Color,
    ) -> Self::Swept {
        Surface::SweptCurve(SweptCurve {
            curve: *self.kind(),
            path: path.into().inner(),
        })
    }
}
