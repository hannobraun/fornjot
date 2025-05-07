use crate::geometry::SurfaceGeometry;

#[derive(Debug)]
pub struct Surface {
    pub geometry: Box<dyn SurfaceGeometry>,
}
