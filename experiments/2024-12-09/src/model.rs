use crate::{
    geometry::{Handle, HandleAny, Sketch},
    math::{Bivector, Plane, Point, Vector},
    topology::sweep::SweepExt,
};

pub fn model() -> HandleAny {
    let top = {
        let sketch =
            Sketch::from([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]);

        let surface = Plane {
            origin: Point::from([0., 0., 0.5]),
            coords: Bivector {
                a: Vector::from([1., 0., 0.]),
                b: Vector::from([0., 1., 0.]),
            },
        };

        sketch.to_face(surface)
    };

    let top = Handle::new(top);

    let solid = top.sweep([0., 0., -1.]);

    HandleAny::new(solid)
}
