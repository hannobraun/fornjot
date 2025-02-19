use crate::{
    geometry::Sketch,
    math::{Bivector, Plane, Point, Vector},
    object::{Handle, HandleAny},
    operations::sweep::SweepExt,
};

pub fn model() -> HandleAny {
    let top = {
        let sketch = Sketch::from([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]]);

        let surface = Plane {
            origin: Point::from([0., 0., 1.]),
            coords: Bivector {
                a: Vector::from([1., 0., 0.]),
                b: Vector::from([0., 1., 0.]),
            },
        };

        let face = sketch.to_face(surface);
        Handle::new(face)
    };

    let solid = top.sweep([0., 0., -2.]);

    HandleAny::new(solid)
}
