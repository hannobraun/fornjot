use std::{collections::BTreeMap, fs::File};

use crate::geometry::{Operation, OpsLog, Vertex};

pub fn export(mesh: &OpsLog) -> anyhow::Result<()> {
    let mut mesh_vertices = Vec::new();
    let mut mesh_triangles = Vec::new();

    mesh.vertices(&mut mesh_vertices);
    mesh.triangles(&mut mesh_triangles);

    let mut indices_by_vertex = BTreeMap::new();

    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    for triangle in mesh_triangles {
        let triangle = triangle.map(|vertex| {
            *indices_by_vertex.entry(vertex).or_insert_with(|| {
                let index = vertices.len();
                vertices.push(vertex);
                index
            })
        });

        triangles.push(triangle);
    }

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices {
            vertex: vertices
                .into_iter()
                .map(|Vertex { point }| point)
                .map(|point| point.coords.map(|coord| coord.value()))
                .map(|[x, y, z]| threemf::model::Vertex { x, y, z })
                .collect(),
        },
        triangles: threemf::model::Triangles {
            triangle: triangles
                .into_iter()
                .map(|[v1, v2, v3]| threemf::model::Triangle { v1, v2, v3 })
                .collect(),
        },
    };

    let output = File::create("output.3mf")?;
    threemf::write(output, mesh)?;

    Ok(())
}
