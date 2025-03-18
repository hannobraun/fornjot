use std::fmt;

use crate::geometry::SurfaceGeometry;

pub struct Surface {
    pub geometry: Box<dyn SurfaceGeometry>,
}

impl fmt::Debug for Surface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Surface")
            .field("geometry", &"Box<dyn SurfaceGeometry>")
            .finish()
    }
}
