use bytemuck::{Pod, Zeroable};

use crate::viewer::graphics::transform::NativeTransform;

use super::transform::Transform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: Transform,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: Transform::identity().to_native(),
            transform_normals: Transform::identity(),
        }
    }
}
