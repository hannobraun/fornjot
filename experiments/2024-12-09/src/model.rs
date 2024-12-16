use crate::{
    geometry::{Shape, Triangle, Vertex},
    storage::Store,
};

pub fn model(shape: &mut Shape) {
    let mut vertices = Store::<Vertex>::new();
    let mut triangles = Store::<Triangle>::new();

    let (a, b, c, d, e, f, g, h) = shape
        .extend_with(&mut vertices)
        .add([-0.5, -0.5, -0.5])
        .add([0.5, -0.5, -0.5])
        .add([-0.5, 0.5, -0.5])
        .add([0.5, 0.5, -0.5])
        .add([-0.5, -0.5, 0.5])
        .add([0.5, -0.5, 0.5])
        .add([-0.5, 0.5, 0.5])
        .add([0.5, 0.5, 0.5])
        .get_new_ops();

    shape
        .extend_with(&mut triangles)
        .add([&a, &e, &g]) // left
        .add([&a, &g, &c])
        .add([&b, &d, &h]) // right
        .add([&b, &h, &f])
        .add([&a, &b, &f]) // front
        .add([&a, &f, &e])
        .add([&c, &h, &d]) // back
        .add([&c, &g, &h])
        .add([&a, &c, &b]) // bottom
        .add([&b, &c, &d])
        .add([&e, &f, &h]) // top
        .add([&e, &h, &g]);
}
