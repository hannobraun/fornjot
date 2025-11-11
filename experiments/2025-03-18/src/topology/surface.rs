use std::rc::Rc;

use crate::geometry::SurfaceGeometry;

#[derive(Debug)]
pub struct Surface {
    pub geometry: Rc<dyn SurfaceGeometry>,
}
