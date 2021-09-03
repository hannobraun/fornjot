use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub type Index = u32;
