use itertools::Itertools;

use crate::{
    geometry::{Shape, Sketch, Triangle},
    math::{Bivector, Plane, Point, Vector},
    storage::Stores,
    topology::Face,
};

pub fn model(shape: &mut Shape) {
    let mut stores = Stores::new();

    let top = {
        let sketch =
            Sketch::from([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]);

        let surface = stores.get().insert(Plane {
            origin: Point::from([0., 0., 0.5]),
            coords: Bivector {
                a: Vector::from([1., 0., 0.]),
                b: Vector::from([0., 1., 0.]),
            },
        });

        Face::new(&sketch, surface, stores.get())
    };
    let bottom = top.flip(stores.get()).translate([0., 0., -1.], &mut stores);

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
        .add([a, e, h]) // left
        .add([a, h, d])
        .add([b, c, g]) // right
        .add([b, g, f])
        .add([a, b, f]) // front
        .add([a, f, e])
        .add([d, g, c]) // back
        .add([d, h, g]);
}
