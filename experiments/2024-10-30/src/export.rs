use std::fs::File;

use crate::mesh::Mesh;

pub fn export(mesh: Mesh) -> anyhow::Result<()> {
    let vertices = mesh
        .vertices
        .into_iter()
        .map(|[x, y, z]| threemf::model::Vertex { x, y, z })
        .collect();

    let triangles = mesh
        .triangles
        .into_iter()
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
