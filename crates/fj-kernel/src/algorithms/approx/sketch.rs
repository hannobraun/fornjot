//! Sketch approximation

use std::collections::BTreeSet;

use crate::objects::Sketch;

use super::{face::FaceApprox, Approx, Tolerance};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        self.faces().approx(tolerance)
    }
}
