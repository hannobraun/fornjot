use crate::{
    geometry::{Shape, Triangle, Vertex},
    storage::Store,
};

pub fn model(shape: &mut Shape) {
    let mut vertices = Store::<Vertex>::new();
    let mut triangles = Store::<Triangle>::new();

    let (a, b, c, d, e, f, g, h) = shape
        .extend_with(&mut vertices)
        .vertex([-0.5, -0.5, -0.5])
        .vertex([0.5, -0.5, -0.5])
        .vertex([-0.5, 0.5, -0.5])
        .vertex([0.5, 0.5, -0.5])
        .vertex([-0.5, -0.5, 0.5])
        .vertex([0.5, -0.5, 0.5])
        .vertex([-0.5, 0.5, 0.5])
        .vertex([0.5, 0.5, 0.5])
        .get_new_ops();

    shape
        .extend_with(&mut triangles)
        .triangle([&a, &e, &g]) // left
        .triangle([&a, &g, &c])
        .triangle([&b, &d, &h]) // right
        .triangle([&b, &h, &f])
        .triangle([&a, &b, &f]) // front
        .triangle([&a, &f, &e])
        .triangle([&c, &h, &d]) // back
        .triangle([&c, &g, &h])
        .triangle([&a, &c, &b]) // bottom
        .triangle([&b, &c, &d])
        .triangle([&e, &f, &h]) // top
        .triangle([&e, &h, &g]);
}
