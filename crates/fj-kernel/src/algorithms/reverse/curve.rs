use crate::objects::GlobalCurve;

use super::Reverse;

impl Reverse for GlobalCurve {
    fn reverse(self) -> Self {
        Self::from_kind(self.kind().reverse())
    }
}
