use bytemuck::{Pod, Zeroable};
use fj_interop::{Index, TriMesh, vertices_to_indexed_vertices};
use fj_math::{Point, Scalar};

#[derive(Debug)]
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Vertices {
    pub fn for_face(points: &[Point<3>]) -> Self {
        let vertices = points
            .iter()
            .map(|point| {
                let [x, y, z] = point.coords.components.map(Scalar::into_f32);

                Vertex {
                    position: [x, y, z],
                    normal: [0., 0., 1.],
                    color: [0., 0., 0., 1.],
                }
            })
            .collect();
        let indices = (0..).take(points.len()).collect();

        Self { vertices, indices }
    }

    pub fn for_mesh(tri_mesh: &TriMesh) -> Self {
        let (vertices, indices) = vertices_to_indexed_vertices(
            tri_mesh.triangles.iter().flat_map(|triangle| {
                let [a, b, c] = triangle.inner.points;

                let normal = (b - a).cross(&(c - a)).normalize();
                let color = triangle.color;

                [a, b, c].map(|point| (point, normal, color))
            }),
            |(point, normal, color)| Vertex {
                position: point.into(),
                normal: normal.into(),
                color: color.0.map(|v| f32::from(v) / 255.0),
            },
        );

        Self { vertices, indices }
    }

    pub fn for_point(point: Point<3>) -> Self {
        let vertices = vec![Vertex {
            position: point.coords.components.map(|coord| coord.into_f32()),
            normal: [0.; 3],
            color: [0., 0., 0., 1.],
        }];
        let indices = vec![0];

        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
