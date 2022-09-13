//! Solid approximation

use std::collections::BTreeSet;

use crate::objects::Solid;

use super::{face::FaceApprox, Approx, Tolerance};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        self.shells()
            .flat_map(|shell| shell.approx(tolerance))
            .collect()
    }
}
