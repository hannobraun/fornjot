use std::fs::File;

use crate::mesh::Mesh;

pub fn export(mesh: &Mesh) -> anyhow::Result<()> {
    let vertices = mesh
        .vertices
        .iter()
        .copied()
        .map(|vertex| vertex.map(Into::into))
        .map(|[x, y, z]| threemf::model::Vertex { x, y, z })
        .collect();

    let triangles = mesh
        .triangles
        .iter()
        .copied()
        .map(|triangle| {
            triangle.map(|index| {
                index.try_into().expect(
                    "Converting `u32` to `usize` must work on all platforms \
                    this software is expected to run on.",
                )
            })
        })
        .map(|[v1, v2, v3]| threemf::model::Triangle { v1, v2, v3 })
        .collect();

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices { vertex: vertices },
        triangles: threemf::model::Triangles {
            triangle: triangles,
        },
    };

    let output = File::create("output.3mf")?;
    threemf::write(output, mesh)?;

    Ok(())
}
