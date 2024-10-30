use std::fs::File;

pub fn export(
    vertices: impl IntoIterator<Item = [f64; 3]>,
    triangles: impl IntoIterator<Item = [usize; 3]>,
) -> anyhow::Result<()> {
    let vertices = vertices
        .into_iter()
        .map(|[x, y, z]| threemf::model::Vertex { x, y, z })
        .collect();

    let triangles = triangles
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
