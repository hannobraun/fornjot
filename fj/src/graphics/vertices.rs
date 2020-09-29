use bytemuck::{Pod, Zeroable};
use decorum::R32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: Array3,
    pub normal: Array3,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Array3(pub [R32; 3]);

impl Array3 {
    pub fn new(value: [f32; 3]) -> Self {
        Self([
            R32::from_inner(value[0]),
            R32::from_inner(value[1]),
            R32::from_inner(value[2]),
        ])
    }

    pub fn into_f32_array(&self) -> [f32; 3] {
        [
            self.0[0].into_inner(),
            self.0[1].into_inner(),
            self.0[2].into_inner(),
        ]
    }
}

pub type Index = u16;
