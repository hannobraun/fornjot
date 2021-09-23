use bytemuck::{Pod, Zeroable};

use crate::math::Matrix;

use super::transform::NativeTransform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
}

impl Default for Uniforms {
    fn default() -> Self {
        let identity = Matrix::<4, 4>::identity();

        let mut transform = [0.0; 16];
        transform.copy_from_slice(identity.data.as_slice());

        let mut transform_normals = [0.0; 16];
        transform_normals.copy_from_slice(identity.data.as_slice());

        Self {
            transform,
            transform_normals,
        }
    }
}
