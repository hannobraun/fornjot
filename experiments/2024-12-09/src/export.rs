use std::{collections::BTreeMap, fs::File};

use crate::geometry::Operation;

pub fn export(shape: &dyn Operation) -> anyhow::Result<()> {
    let tri_mesh = shape.tri_mesh();

    let mut indices_by_vertex = BTreeMap::new();

    let mut points = Vec::new();
    let mut triangles = Vec::new();

    for triangle in tri_mesh.triangles {
        let triangle = triangle.points.map(|point| {
            *indices_by_vertex.entry(point).or_insert_with(|| {
                let index = points.len();
                points.push(point);
                index
            })
        });

        triangles.push(triangle);
    }

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices {
            vertex: points
                .into_iter()
                .map(|point| point.coords.components.map(|coord| coord.value()))
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
