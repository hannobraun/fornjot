use std::fs::File;

use crate::geometry::{Mesh, Operation, Vertex};

pub fn export(mesh: &Mesh) -> anyhow::Result<()> {
    let mut vertices = Vec::new();

    mesh.vertices(&mut vertices);
    let triangles = mesh.triangles();

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices {
            vertex: vertices
                .into_iter()
                .map(|Vertex { point: [x, y, z] }| threemf::model::Vertex {
                    x,
                    y,
                    z,
                })
                .collect(),
        },
        triangles: threemf::model::Triangles {
            triangle: triangles
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
