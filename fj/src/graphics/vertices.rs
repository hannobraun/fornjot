use bytemuck::{Pod, Zeroable};
use decorum::R32;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [R32; 3],
    pub normal: [R32; 3],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub type Index = u16;
