use itertools::Itertools;

use crate::{
    geometry::{Shape, Sketch, Triangle},
    math::{Bivector, Plane, Point, Vector},
    storage::Stores,
    topology::Face,
};

pub fn model(shape: &mut Shape) {
    let mut stores = Stores::new();

    let bottom = stores.get().insert(Plane {
        origin: Point::from([0., 0., -0.5]),
        coords: Bivector {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., -1., 0.]),
        },
    });
    let top = stores.get().insert(Plane {
        origin: Point::from([0., 0., 0.5]),
        coords: Bivector {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        },
    });

    let sketch =
        Sketch::from([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]);

    let [bottom, top] =
        [bottom, top].map(|plane| Face::new(&sketch, plane, stores.get()));

    let (bottom, top) = shape
        .extend_with(stores.get::<Face>())
        .add(bottom)
        .add(top)
        .get_added();

    let [a, b, c, d] = bottom.vertices().collect_array().unwrap();
    let [e, f, g, h] = top.vertices().collect_array().unwrap();

    let [a, b, c, d, e, f, g, h] =
        [a, b, c, d, e, f, g, h].map(|vertex| vertex.point);

    shape
        .extend_with(stores.get::<Triangle>())
        .add([d, e, h]) // left
        .add([d, h, a])
        .add([c, b, g]) // right
        .add([c, g, f])
        .add([d, c, f]) // front
        .add([d, f, e])
        .add([a, g, b]) // back
        .add([a, h, g]);
}
