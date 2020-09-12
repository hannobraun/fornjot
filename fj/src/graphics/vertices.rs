use bytemuck::{Pod, Zeroable};
use decorum::R32;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub position: Array3,
    pub normal: Array3,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Array3(pub [R32; 3]);

pub type Index = u16;
