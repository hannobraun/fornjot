use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{curve::Line, surface::Plane},
    operations::{sketch::Sketch, sweep},
    store::Store,
};

mod debug;
mod geometry;
mod helpers;
mod operations;
mod store;
mod topology;

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
    let mut vertices = Store::default();
    let mut half_edges = Store::default();
    let mut faces = Store::default();
    let mut solids = Store::default();

    let left_front_bottom_outer = vertices.push([0., 0., 0.]);
    let left_front_bottom_inner = vertices.push([0.25, 0.25, 0.]);

    let radius = Scalar::from(0.5);
    let tolerance = Scalar::from(0.001);

    let bottom = {
        Sketch::start_at([0., 0.])
            .line_to([1., 0.])
            .line_to([1., 1.])
            .line_to([0., 1.])
            .line_to_with(
                Line {
                    end: Point::from([0., 0.]),
                },
                left_front_bottom_outer,
            )
            .line_to_with(
                Line {
                    end: Point::from([0.25, 0.25]),
                },
                left_front_bottom_inner,
            )
            .arc_to([0.25, 0.75], radius, tolerance)
            .arc_to([0.75, 0.75], radius, tolerance)
            .arc_to([0.75, 0.25], radius, tolerance)
            .arc_to_at(
                [0.25, 0.25],
                radius,
                tolerance,
                left_front_bottom_inner,
            )
            .line_to_with(
                Line {
                    end: Point::from([0., 0.]),
                },
                left_front_bottom_outer,
            )
            .into_face(
                Plane {
                    origin: Point::from([0., 0., 0.]),
                    axes: [
                        Vector::from([0., 1., 0.]),
                        Vector::from([1., 0., 0.]),
                    ],
                },
                &mut vertices,
                &mut half_edges,
                &mut faces,
            )
    };

    let cube = sweep::face_to_solid(
        bottom,
        [0., 0., 1.],
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
