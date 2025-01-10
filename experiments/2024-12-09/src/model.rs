use crate::{
    geometry::{Shape, Sketch, Triangle},
    math::{Bivector, Plane, Point, Vector},
    storage::Store,
    topology::Vertex,
};

pub fn model(shape: &mut Shape) {
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

    let (a, b, c, d, e, f, g, h) = shape
        .extend_with(&mut vertices)
        .add(bottom.point_from_local(sketch.points[0]))
        .add(bottom.point_from_local(sketch.points[1]))
        .add(bottom.point_from_local(sketch.points[2]))
        .add(bottom.point_from_local(sketch.points[3]))
        .add(top.point_from_local(sketch.points[0]))
        .add(top.point_from_local(sketch.points[1]))
        .add(top.point_from_local(sketch.points[2]))
        .add(top.point_from_local(sketch.points[3]))
        .get_added();

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
        .add([a, h, g])
        .add([d, a, c]) // bottom
        .add([c, a, b])
        .add([e, f, g]) // top
        .add([e, g, h]);
}
