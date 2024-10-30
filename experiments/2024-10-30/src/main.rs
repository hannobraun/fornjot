use std::fs::File;

fn main() -> anyhow::Result<()> {
    let vertices = [
        [-0.5, -0.5, -0.5], // 0
        [0.5, -0.5, -0.5],  // 1
        [-0.5, 0.5, -0.5],  // 2
        [0.5, 0.5, -0.5],   // 3
        [-0.5, -0.5, 0.5],  // 4
        [0.5, -0.5, 0.5],   // 5
        [-0.5, 0.5, 0.5],   // 6
        [0.5, 0.5, 0.5],    // 7
    ];

    let triangles = [
        [0, 4, 6], // left
        [0, 6, 2],
        [1, 3, 7], // right
        [1, 7, 5],
        [0, 1, 5], // front
        [0, 5, 4],
        [2, 7, 3], // back
        [2, 6, 7],
        [0, 2, 1], // bottom
        [1, 2, 3],
        [4, 5, 7], // top
        [4, 7, 6],
    ];

    export(vertices, triangles)?;

    Ok(())
}

fn export(
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
