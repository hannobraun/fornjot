use crate::{
    geometry::{Sketch, TriMesh},
    math::{Bivector, Plane, Point, Vector},
    object::{Handle, HandleAny, Object},
    operations::sweep::SweepExt,
    topology::surface::Surface,
};

pub fn model() -> TriMesh {
    let top = {
        let sketch = Sketch::from([
            // outer boundary
            [-1., -1.],
            [1., -1.],
            [1., 1.],
            [-1., 1.],
            // connection to inner boundary
            [-1., -1.],
            // inner boundary
            [-0.5, -0.5],
            [-0.5, 0.5],
            [0.5, 0.5],
            [0.5, -0.5],
            // connection to outer boundary
            [-0.5, -0.5],
            // half-edge between last and first vertex is implicit, so we're done here
        ]);

        let surface = Handle::new(Surface {
            geometry: Box::new(Plane {
                origin: Point::from([0., 0., 1.]),
                coords: Bivector {
                    a: Vector::from([1., 0., 0.]),
                    b: Vector::from([0., 1., 0.]),
                },
            }),
        });

        let face = sketch.to_face(surface);
        Handle::new(face)
    };

    let solid = top.sweep([0., 0., -2.]);

    HandleAny::new(solid).tri_mesh()
}
