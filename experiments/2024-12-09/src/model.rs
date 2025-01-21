use itertools::Itertools;

use crate::{
    geometry::{Shape, Sketch},
    math::{Bivector, Plane, Point, Vector},
    storage::Stores,
    topology::{Face, Solid},
};

pub fn model() -> Shape {
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

        sketch.to_face(surface, stores.get())
    };
    let bottom = top.flip(stores.get()).translate([0., 0., -1.], &mut stores);

    let [a, b, c, d] = bottom.vertices().collect_array().unwrap();
    let [e, f, g, h] = top.vertices().collect_array().unwrap();

    let [left, right, front, back] =
        [[a, e, h, d], [b, c, g, f], [a, b, f, e], [c, d, h, g]].map(
            |[q, r, s, t]| {
                let surface = stores.get().insert(Plane::from_points(
                    [q, r, s].map(|vertex| vertex.point),
                ));
                Face::new(surface, [q, r, s, t].map(|vertex| vertex.clone()))
            },
        );

    let solid = Solid::new(
        [bottom, top, left, right, front, back]
            .map(|face| stores.get().insert(face)),
    );

    let mut shape = Shape::default();
    shape.extend_with(stores.get::<Solid>()).add(solid);
    shape
}
