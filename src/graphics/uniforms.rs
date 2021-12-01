use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;

use super::transform::NativeTransform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
}

impl Default for Uniforms {
    fn default() -> Self {
        let identity = Matrix4::identity();

        let mut transform = NativeTransform([0.0; 16]);
        transform.0.copy_from_slice(identity.data.as_slice());

        let mut transform_normals = NativeTransform([0.0; 16]);
        transform_normals
            .0
            .copy_from_slice(identity.data.as_slice());

        Self {
            transform,
            transform_normals,
        }
    }
}
