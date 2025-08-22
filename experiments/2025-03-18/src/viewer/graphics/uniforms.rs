use bytemuck::{Pod, Zeroable};

use super::transform::Transform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: Transform,
    pub transform_normals: Transform,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
            transform_normals: Transform::identity(),
        }
    }
}
