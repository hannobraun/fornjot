use bytemuck::{Pod, Zeroable};
use euclid::Transform3D;

#[derive(Clone, Copy)]
pub struct Uniforms {
    pub transform: [[f32; 4]; 4],
}

impl Default for Uniforms {
    fn default() -> Self {
        let identity = Transform3D::<f32, (), ()>::identity();

        Self {
            transform: identity.to_arrays(),
        }
    }
}

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}
