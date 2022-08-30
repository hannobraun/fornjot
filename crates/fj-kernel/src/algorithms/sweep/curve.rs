use crate::objects::{GlobalCurve, Surface, SweptCurve};

use super::Sweep;

impl Sweep for GlobalCurve {
    type Swept = Surface;

    fn sweep(
        self,
        path: impl Into<super::Path>,
        _: impl Into<crate::algorithms::approx::Tolerance>,
        _: fj_interop::mesh::Color,
    ) -> Self::Swept {
        Surface::SweptCurve(SweptCurve {
            curve: *self.kind(),
            path: path.into().inner(),
        })
    }
}
