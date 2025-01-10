use itertools::Itertools;

use crate::{
    geometry::{Shape, Sketch, Triangle},
    math::{Bivector, Plane, Point, Vector},
    storage::Store,
    topology::{Face, Vertex},
};

pub fn model(shape: &mut Shape) {
    let mut faces = Store::<Face>::new();
    let mut surfaces = Store::<Plane>::new();
    let mut vertices = Store::<Vertex>::new();
    let mut triangles = Store::<Triangle>::new();

    let bottom = surfaces.insert(Plane {
        origin: Point::from([0., 0., -0.5]),
        coords: Bivector {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., -1., 0.]),
        },
    });
    let top = surfaces.insert(Plane {
        origin: Point::from([0., 0., 0.5]),
        coords: Bivector {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        },
    });

    let sketch =
        Sketch::from([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]);

    let [bottom, top] =
        [bottom, top].map(|plane| Face::new(&sketch, plane, &mut vertices));

    let (bottom, top) = shape
        .extend_with(&mut faces)
        .add(bottom)
        .add(top)
        .get_added();

    let [a, b, c, d] = bottom.vertices.iter().collect_array().unwrap();
    let [e, f, g, h] = top.vertices.iter().collect_array().unwrap();

    let [a, b, c, d, e, f, g, h] =
        [a, b, c, d, e, f, g, h].map(|vertex| vertex.point);

    shape
        .extend_with(&mut triangles)
        .add([d, e, h]) // left
        .add([d, h, a])
        .add([c, b, g]) // right
        .add([c, g, f])
        .add([d, c, f]) // front
        .add([d, f, e])
        .add([a, g, b]) // back
        .add([a, h, g]);
}
