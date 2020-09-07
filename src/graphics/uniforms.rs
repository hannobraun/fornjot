use bytemuck::{Pod, Zeroable};
use euclid::Transform3D;

use crate::transform::NativeTransform;

#[derive(Clone, Copy)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
}

impl Default for Uniforms {
    fn default() -> Self {
        let identity = Transform3D::<f32, (), ()>::identity();

        Self {
            transform: identity.to_arrays(),
            transform_normals: identity.to_arrays(),
        }
    }
}

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}
