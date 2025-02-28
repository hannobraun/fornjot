use std::fmt;

use crate::math::Plane;

pub struct Surface {
    pub geometry: Plane,
}

impl fmt::Debug for Surface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Surface")
            .field("geometry", &self.geometry)
            .finish()
    }
}
