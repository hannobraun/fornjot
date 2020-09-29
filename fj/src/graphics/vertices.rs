use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: Array3,
    pub normal: Array3,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Array3(pub [f32; 3]);

pub type Index = u16;
