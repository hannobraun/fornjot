use fj_interop::{Tolerance, TriMesh};
use fj_math::Vector;

use crate::{
    geometry::{FloatingCurve, Line, Sketch, SweptCurve},
    handle::Handle,
    operations::sweep::Sweep,
    topology::surface::Surface,
    viewer::ViewerHandle,
};

pub fn model(viewer: &ViewerHandle) -> TriMesh {
    let tolerance = Tolerance::from(0.001);

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
            .arc_from([-0.5, -0.5], 1.)
            .arc_from([-0.5, 0.5], 1.)
            .arc_from([0.5, 0.5], 1.)
            .arc_from([0.5, -0.5], 1.)
            // connection to outer boundary
            .line_from([-0.5, -0.5]);
        // The connection between the last and first points is implicit, so
        // we're done here.

        let surface = Handle::new(Surface {
            geometry: Box::new(SweptCurve::plane_from_coord_system(
                [0., 0., 1.],
                [[1., 0., 0.], [0., 1., 0.]],
            )),
        });

        let face = sketch.to_face(surface, tolerance);
        Handle::new(face)
    };

    let solid = {
        let solid = top.sweep(
            FloatingCurve::new(Line {
                direction: Vector::from([0., 0., -2.]),
            }),
            [1.],
            tolerance,
        );

        solid.to_tri_mesh(tolerance)
    };

    viewer.open_window().display_mesh(solid.clone());

    solid
}
