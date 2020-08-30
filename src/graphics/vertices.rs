#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    [-0.5, -0.5, -0.5], // 0
    [-0.5, -0.5,  0.5], // 1
    [-0.5,  0.5, -0.5], // 2
    [-0.5,  0.5,  0.5], // 3
    [ 0.5, -0.5, -0.5], // 4
    [ 0.5, -0.5,  0.5], // 5
    [ 0.5,  0.5, -0.5], // 6
    [ 0.5,  0.5,  0.5], // 7
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

pub type Vertex = [f32; 3];
pub type Index = u16;
