use crate::objects::Surface;

use super::Reverse;

impl Reverse for Surface {
    fn reverse(self) -> Self {
        match self {
            Self::SweptCurve(surface) => Self::SweptCurve(surface.reverse()),
        }
    }
}
