use fj_math::Vector;

use crate::{
    objects::{Curve, Surface},
    stores::Stores,
};

use super::Sweep;

impl Sweep for Curve {
    type Swept = Surface;

    fn sweep(self, path: impl Into<Vector<3>>, _: &Stores) -> Self::Swept {
        Surface::new(self.global_form().path(), path)
    }
}
