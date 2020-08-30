use bytemuck::{Pod, Zeroable};

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    // left
    Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0,  0.0,  0.0] }, // 0
    Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0,  0.0,  0.0] }, // 1
    Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0,  0.0,  0.0] }, // 2
    Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0,  0.0,  0.0] }, // 3

    // right
    Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 1.0,  0.0,  0.0] }, // 4
    Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 1.0,  0.0,  0.0] }, // 5
    Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 1.0,  0.0,  0.0] }, // 6
    Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 1.0,  0.0,  0.0] }, // 7

    // front
    Vertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0] }, // 8
    Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0] }, // 9
    Vertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0] }, // 10
    Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0] }, // 11

    // back
    Vertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0] }, // 12
    Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0] }, // 13
    Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0] }, // 14
    Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0] }, // 15

    // bottom
    Vertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0] }, // 16
    Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0] }, // 17
    Vertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0] }, // 18
    Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0] }, // 19

    // top
    Vertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0] }, // 20
    Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0] }, // 21
    Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0] }, // 22
    Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0] }, // 23
];

#[rustfmt::skip]
pub const INDICES: &[Index] = &[
    // left
    0, 2, 1,
    1, 2, 3,

    // right
    4, 5, 6,
    5, 7, 6,

    // front
    8, 9, 10,
    9, 11, 10,

    // back
    12, 14, 13,
    13, 14, 15,

    // bottom
    16, 18, 19,
    16, 19, 17,

    // top
    20, 21, 23,
    20, 23, 22,
];

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub type Index = u16;
