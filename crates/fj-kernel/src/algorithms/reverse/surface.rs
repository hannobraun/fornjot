use crate::objects::Surface;

use super::Reverse;

impl Reverse for Surface {
    /// Reverse the surface
    ///
    /// # Implementation Note
    ///
    /// Right now, the orientation of a face is defined by the orientation of
    /// its surface. This leads to some complications. See this issue for
    /// context:
    /// <https://github.com/hannobraun/Fornjot/issues/695>
    ///
    /// It would probably be much better, if `Surface`s were without
    /// orientation, and the orientation of a face were defined by the direction
    /// of the half-edges that bound it.
    ///
    /// Please note that, as of this writing, half-edges don't really exist as a
    /// concept in the kernel. We kind of treat `Edge` as a half-edge, but in an
    /// inconsistent way that causes problems. This issue has some context on
    /// that:
    /// <https://github.com/hannobraun/Fornjot/issues/993>
    ///
    /// So, in conclusion, in a perfect world, this implementation would not
    /// exist.
    fn reverse(self) -> Self {
        match self {
            Self::SweptCurve(surface) => Self::SweptCurve(surface.reverse()),
        }
    }
}
