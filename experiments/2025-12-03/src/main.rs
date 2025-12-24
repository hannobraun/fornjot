use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Vector};

use crate::{
    objects::{
        geometry::{Geometry, Triangle, Vertex},
        topology::Faces,
    },
    operations::{
        sketch::{Sketch, Surface},
        sweep,
    },
    store::Store,
};

mod objects;
mod operations;
mod store;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        let tri_mesh = model();
        viewer.open_window().display_mesh(tri_mesh.clone());
        tri_mesh
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model() -> TriMesh {
    let mut geometry = Geometry::default();

    // Topology
    let mut half_edges = Store::default();
    let mut faces = Faces::default();
    let mut solids = Store::default();

    let left_front_bottom_outer = geometry.vertices.push(Vertex {
        point: geometry.points.push([0., 0., 0.]),
        position: Point::from([0., 0., 0.]),
    });
    let left_front_bottom_inner = geometry.vertices.push(Vertex {
        point: geometry.points.push([0.25, 0.25, 0.]),
        position: Point::from([0.25, 0.25, 0.]),
    });

    let bottom = Sketch::start_at([0., 0.])
        .line_to([1., 0.])
        .line_to([1., 1.])
        .line_to([0., 1.])
        .line_to_vertex([0., 0.], left_front_bottom_outer)
        .line_to_vertex([0.25, 0.25], left_front_bottom_inner)
        .line_to([0.25, 0.75])
        .line_to([0.75, 0.75])
        .line_to([0.75, 0.25])
        .line_to_vertex([0.25, 0.25], left_front_bottom_inner)
        .line_to_vertex([0., 0.], left_front_bottom_outer)
        .into_face(
            Surface {
                origin: Point::from([0., 0., 0.]),
                axes: [Vector::from([0., 1., 0.]), Vector::from([1., 0., 0.])],
            },
            &mut geometry,
            &mut half_edges,
            &mut faces,
        );

    let cube = sweep::face_to_solid(
        bottom,
        [0., 0., 1.],
        &mut geometry,
        &mut half_edges,
        &mut faces,
        &mut solids,
    );

    let mut tri_mesh = TriMesh::new();

    let triangles = solids[cube]
        .boundary
        .iter()
        .copied()
        .flat_map(|f0123| faces[f0123].triangles.iter().copied())
        .map(|t012| &geometry.triangles[t012]);

    for &Triangle {
        points: _,
        vertices: [a, b, c],
    } in triangles
    {
        tri_mesh.triangles.push(MeshTriangle {
            inner: fj_math::Triangle::from_points([
                geometry.vertices[a].position,
                geometry.vertices[b].position,
                geometry.vertices[c].position,
            ]),
            is_internal: false,
            color: Color::default(),
        });
    }

    tri_mesh
}
