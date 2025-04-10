use fj_interop::TriMesh;
use fj_math::{Line, Vector};
use fj_viewer::Viewer;

use crate::{
    geometry::{Sketch, SweptCurve, ToTriMesh},
    handle::Handle,
    operations::sweep::SweepExt,
    topology::surface::Surface,
};

pub fn model(viewer: &Viewer) -> TriMesh {
    let top = {
        let sketch = Sketch::new()
            // outer boundary
            .line_from([-1., -1.])
            .line_from([1., -1.])
            .line_from([1., 1.])
            .line_from([-1., 1.])
            // connection to inner boundary
            .line_from([-1., -1.])
            // inner boundary
            .line_from([-0.5, -0.5])
            .line_from([-0.5, 0.5])
            .line_from([0.5, 0.5])
            .line_from([0.5, -0.5])
            // connection to outer boundary
            .line_from([-0.5, -0.5]);
        // connection between last and first point is implicit, so we're
        // done here

        let surface = Handle::new(Surface {
            geometry: Box::new(SweptCurve {
                curve: Box::new(Line::from_origin_and_direction(
                    [0., 0., 1.],
                    [1., 0., 0.],
                )),
                path: Vector::from([0., 1., 0.]),
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
