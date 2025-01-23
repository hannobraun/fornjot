use itertools::Itertools;

use crate::{
    geometry::{AnyOp, Sketch},
    math::{Bivector, Plane, Point, Vector},
    storage::Stores,
    topology::{Face, Solid},
};

pub fn model() -> AnyOp {
    let mut stores = Stores::new();

    let top = {
        let sketch =
            Sketch::from([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]);

        let surface = stores.surfaces.insert(Plane {
            origin: Point::from([0., 0., 0.5]),
            coords: Bivector {
                a: Vector::from([1., 0., 0.]),
                b: Vector::from([0., 1., 0.]),
            },
        });

        sketch.to_face(surface, &mut stores.vertices)
    };
    let bottom = top.flip(&mut stores.surfaces).translate(
        [0., 0., -1.],
        &mut stores.surfaces,
        &mut stores.vertices,
    );

    let [a, b, c, d] = bottom.vertices().collect_array().unwrap();
    let [e, f, g, h] = top.vertices().collect_array().unwrap();

    let [left, right, front, back] =
        [[a, e, h, d], [b, c, g, f], [a, b, f, e], [c, d, h, g]].map(
            |[q, r, s, t]| {
                let surface = stores.surfaces.insert(Plane::from_points(
                    [q, r, s].map(|vertex| vertex.point),
                ));
                Face::new(surface, [q, r, s, t].map(|vertex| vertex.clone()))
            },
        );

    let solid = Solid::new(
        [bottom, top, left, right, front, back]
            .map(|face| stores.faces.insert(face)),
    );

    AnyOp::new(solid)
}
