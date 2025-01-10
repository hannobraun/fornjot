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
            b: Vector::from([0., 1., 0.]),
        },
    });
    let top = surfaces.insert(bottom.translate([0., 0., 1.]));

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
        .add([a, e, h]) // left
        .add([a, h, d])
        .add([b, c, g]) // right
        .add([b, g, f])
        .add([a, b, f]) // front
        .add([a, f, e])
        .add([d, g, c]) // back
        .add([d, h, g])
        .add([a, d, b]) // bottom
        .add([b, d, c])
        .add([e, f, g]) // top
        .add([e, g, h]);
}
