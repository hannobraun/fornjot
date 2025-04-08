use fj_interop::TriMesh;
use fj_math::{Point, Vector};
use fj_viewer::Viewer;

use crate::{
    geometry::{Plane, Sketch, ToTriMesh},
    handle::Handle,
    operations::sweep::SweepExt,
    topology::surface::Surface,
};

pub fn model(viewer: &Viewer) -> TriMesh {
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
            // half-edge between last and first vertex is implicit, so we're
            // done here
        ]);

        let surface = Handle::new(Surface {
            geometry: Box::new(Plane {
                origin: Point::from([0., 0., 1.]),
                u: Vector::from([1., 0., 0.]),
                v: Vector::from([0., 1., 0.]),
            }),
        });

        let face = sketch.to_face(surface);
        Handle::new(face)
    };

    viewer.display(top.to_tri_mesh());

    let solid = top.sweep([0., 0., -2.]);
    viewer.display(solid.to_tri_mesh());

    solid.to_tri_mesh()
}
