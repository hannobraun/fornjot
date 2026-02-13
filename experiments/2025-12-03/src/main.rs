use fj_core::new::{
    geometry::{Arc, Plane},
    operations::{Sketch, Sweep},
    topology::Topology,
};
use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Scalar, Vector};

mod debug;

use debug::DEBUG_WINDOW;

fn main() -> anyhow::Result<()> {
    let tri_mesh = model();

    fj_viewer::make_viewer_and_spawn_thread({
        let tri_mesh = tri_mesh.clone();

        |viewer| {
            DEBUG_WINDOW.initialize(&viewer);
            viewer.open_window().display_mesh(tri_mesh);
        }
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model() -> TriMesh {
    let Topology {
        mut faces,
        mut half_edges,
        mut solids,
        mut vertices,
    } = Topology::new();

    let mut sweep = Sweep::new();

    let left_front_bottom_outer = vertices.push([0., 0., 0.]);
    let left_front_bottom_inner = vertices.push([0.25, 0.25, 0.]);

    let radius = Scalar::from(1.0);
    let tolerance = Scalar::from(0.001);

    let bottom = Sketch::new()
        .arc_to(radius, tolerance, [1., 0.])
        .arc_to(radius, tolerance, [1., 1.])
        .arc_to(radius, tolerance, [0., 1.])
        .arc_to_at(radius, tolerance, [0., 0.], left_front_bottom_outer)
        .line_to_at([0.25, 0.25], left_front_bottom_inner)
        .line_to([0.25, 0.75])
        .line_to([0.75, 0.75])
        .line_to([0.75, 0.25])
        .line_to_at([0.25, 0.25], left_front_bottom_inner)
        .line_to_at([0., 0.], left_front_bottom_outer)
        .into_face(
            Plane {
                origin: Point::from([0., 0., 0.]),
                axes: [Vector::from([0., 1., 0.]), Vector::from([1., 0., 0.])],
            },
            &mut vertices,
            &mut half_edges,
            &mut faces,
        );

    let cube = sweep.face_to_solid(
        bottom,
        &Arc::to([0., 0., 1.], [1., 1., 1.], tolerance),
        &mut vertices,
        &mut half_edges,
        &mut faces,
        &mut solids,
    );

    let triangles = solids[cube]
        .boundary
        .iter()
        .flat_map(|&face| &faces[face].approx);

    let mut tri_mesh = TriMesh::new();

    for &triangle in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: triangle,
            is_internal: false,
            color: Color::default(),
        });
    }

    tri_mesh
}
