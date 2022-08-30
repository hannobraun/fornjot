use crate::objects::{Curve, GlobalCurve};

use super::Reverse;

impl Reverse for Curve {
    fn reverse(self) -> Self {
        Curve::new(self.kind().reverse(), self.global().reverse())
    }
}

impl Reverse for GlobalCurve {
    fn reverse(self) -> Self {
        Self::from_kind(self.kind().reverse())
    }
}
