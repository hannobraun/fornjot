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

    let [bottom, top] = [bottom, top].map(|face| stores.faces.insert(face));

    let side_faces = bottom
        .half_edges()
        .zip(top.half_edges())
        .map(|([q, r], [t, s])| {
            let surface = stores.surfaces.insert(Plane::from_points(
                [q, r, s].map(|vertex| vertex.point),
            ));
            let face =
                Face::new(surface, [q, r, s, t].map(|vertex| vertex.clone()));
            stores.faces.insert(face)
        })
        .collect::<Vec<_>>();

    let solid = Solid::new([bottom, top].into_iter().chain(side_faces));

    AnyOp::new(solid)
}
