use bytemuck::{Pod, Zeroable};

use super::camera::NativeTransform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: NativeTransform::identity(),
            transform_normals: NativeTransform::identity(),
        }
    }
}
