use bytemuck::{Pod, Zeroable};
use nalgebra::{storage::ContiguousStorage as _, Matrix4};

use super::transform::NativeTransform;

#[derive(Clone, Copy)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
}

impl Default for Uniforms {
    fn default() -> Self {
        let identity = Matrix4::<f32>::identity();

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

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}
