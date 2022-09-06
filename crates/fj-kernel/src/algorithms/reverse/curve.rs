use crate::objects::{Curve, GlobalCurve};

use super::Reverse;

impl Reverse for Curve {
    /// Reverse the curve
    ///
    /// # Implementation Note
    ///
    /// Right now, the orientation of a face is defined by the orientation of
    /// its surface. By extension, the orientation of a surface is defined by
    /// the orientation of the curve it was swept from. This leads to some
    /// complications. See this issue for context:
    /// <https://github.com/hannobraun/Fornjot/issues/695>
    ///
    /// It would probably be much better, if `Surface`s were without
    /// orientation, which would then make it possible for curves to be without
    /// direction. Then this implementation would not exist.
    fn reverse(self) -> Self {
        Curve::new(
            *self.surface(),
            self.kind().reverse(),
            self.global_form().reverse(),
        )
    }
}

impl Reverse for GlobalCurve {
    /// Reverse the curve
    ///
    /// # Implementation Note
    ///
    /// Right now, the orientation of a face is defined by the orientation of
    /// its surface. By extension, the orientation of a surface is defined by
    /// the orientation of the curve it was swept from. This leads to some
    /// complications. See this issue for context:
    /// <https://github.com/hannobraun/Fornjot/issues/695>
    ///
    /// It would probably be much better, if `Surface`s were without
    /// orientation, which would then make it possible for curves to be without
    /// direction. Then this implementation would not exist.
    fn reverse(self) -> Self {
        Self::from_kind(self.kind().reverse())
    }
}
