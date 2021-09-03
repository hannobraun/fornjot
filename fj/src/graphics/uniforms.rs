use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;

use super::transform::NativeTransform;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Uniforms {
    pub transform: NativeTransform,
    pub transform_normals: NativeTransform,
    pub invert_color: bool,

    /// Padding to make this struct match its WGSL equivalent
    ///
    /// WGSL seems to size its structs in increments of 16, so we need some
    /// padding here to prevent a validation error.
    pub _padding: [u8; 15],
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
            invert_color: false,
            _padding: [0; 15],
        }
    }
}

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}
