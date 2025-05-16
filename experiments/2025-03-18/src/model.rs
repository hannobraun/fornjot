use fj_interop::{Tolerance, TriMesh};
use fj_math::Vector;
use fj_viewer::Viewer;

use crate::{
    extra::triangulate::ProjectedFace,
    geometry::{FloatingCurve, Line, Sketch, SweptCurve, ToTriMesh},
    handle::Handle,
    operations::sweep::SweepExt,
    topology::surface::Surface,
};

pub fn model(viewer: &Viewer) -> TriMesh {
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

        let face = sketch.to_face(surface);
        Handle::new(face)
    };

    viewer.display_face(
        ProjectedFace::new(&top, tolerance)
            .points
            .into_iter()
            .map(|point| point.point_surface)
            .collect(),
    );
    viewer.display_model(top.to_tri_mesh(tolerance));

    let solid = top.sweep(
        FloatingCurve::new(Line {
            direction: Vector::from([0., 0., -2.]),
        }),
        [1.],
    );
    viewer.display_model(solid.to_tri_mesh(tolerance));

    solid.to_tri_mesh(tolerance)
}
