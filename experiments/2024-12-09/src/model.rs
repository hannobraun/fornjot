use crate::{
    geometry::{AnyOp, Sketch},
    math::{Bivector, Plane, Point, Vector},
    storage::Stores,
    topology::Solid,
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

    let [bottom, top] = [bottom, top].map(|face| stores.faces.insert(face));

    let solid = Solid::connect_faces(
        [bottom, top],
        &mut stores.faces,
        &mut stores.surfaces,
    );

    AnyOp::new(solid)
}
