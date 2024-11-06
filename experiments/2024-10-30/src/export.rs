use std::fs::File;

use crate::geometry::{Mesh, Operation, Vertex};

pub fn export(mesh: &Mesh) -> anyhow::Result<()> {
    let mut mesh_vertices = Vec::new();
    let mut mesh_triangles = Vec::new();

    mesh.vertices(&mut mesh_vertices);
    mesh.triangles(&mut mesh_triangles);

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices {
            vertex: mesh_vertices
                .into_iter()
                .map(|Vertex { point }| point)
                .map(|point| point.map(|coord| coord.value()))
                .map(|[x, y, z]| threemf::model::Vertex { x, y, z })
                .collect(),
        },
        triangles: threemf::model::Triangles {
            triangle: mesh_triangles
                .into_iter()
                .map(|triangle| {
                    triangle.map(|index| {
                        index.try_into().expect(
                            "Converting `u32` to `usize` must work on all \
                            platforms this software is expected to run on.",
                        )
                    })
                })
                .map(|[v1, v2, v3]| threemf::model::Triangle { v1, v2, v3 })
                .collect(),
        },
    };

    let output = File::create("output.3mf")?;
    threemf::write(output, mesh)?;

    Ok(())
}
