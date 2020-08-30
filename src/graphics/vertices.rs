use bytemuck::{Pod, Zeroable};

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, -0.5] }, // 0
    Vertex { position: [-0.5, -0.5,  0.5] }, // 1
    Vertex { position: [-0.5,  0.5, -0.5] }, // 2
    Vertex { position: [-0.5,  0.5,  0.5] }, // 3
    Vertex { position: [ 0.5, -0.5, -0.5] }, // 4
    Vertex { position: [ 0.5, -0.5,  0.5] }, // 5
    Vertex { position: [ 0.5,  0.5, -0.5] }, // 6
    Vertex { position: [ 0.5,  0.5,  0.5] }, // 7
];

#[rustfmt::skip]
pub const INDICES: &[Index] = &[
    // left
    0, 1, 3,
    0, 3, 2,

    // right
    4, 7, 5,
    4, 6, 7,

    // front
    0, 4, 1,
    1, 4, 5,

    // back
    2, 3, 6,
    3, 7, 6,

    // bottom
    0, 2, 4,
    2, 6, 4,

    // top
    1, 5, 3,
    3, 5, 7,
];

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub type Index = u16;
