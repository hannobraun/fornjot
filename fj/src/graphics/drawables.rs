use super::{geometry::Geometry, pipelines::Pipeline};

pub struct Drawable<'r> {
    pub geometry: &'r Geometry,
    pub pipeline: &'r Pipeline,
}
