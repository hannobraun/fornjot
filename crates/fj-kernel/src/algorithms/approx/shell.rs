//! Shell approximation

use std::collections::BTreeSet;

use crate::objects::Shell;

use super::{face::FaceApprox, Approx, Tolerance};

impl Approx for &Shell {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        self.faces().approx(tolerance)
    }
}
