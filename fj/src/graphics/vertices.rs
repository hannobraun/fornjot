use bytemuck::{Pod, Zeroable};
use decorum::R32;
use indexmap::IndexMap;

use crate::{geometry::isosurface::grid::Grid, mesh, types::Index, util};

#[derive(Debug)]
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Vertices {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

impl From<Grid> for Vertices {
    fn from(grid: Grid) -> Self {
        let mut vertices = util::Vertices::new();
        let mut indices = Vec::new();

        for edge in grid.all_edges() {
            let a = vertices.index_for_vertex(edge.a);
            let b = vertices.index_for_vertex(edge.b);

            indices.push(a);
            indices.push(b);
        }

        let vertices = vertices
            .iter()
            .map(|vertex| {
                // TASK: For some reason, there are yellow grid vertices on the
                //       top and bottom of the spacer hole. The alpha values in
                //       the hole also seem to be wrong. Something is wrong.

                let resolution = grid.descriptor().resolution;

                let threshold_a = (resolution, 1.0);
                let threshold_b = (resolution * 3.0, 0.2);

                let color = if vertex.distance.abs() > threshold_a.0 {
                    if vertex.distance >= 0.0 {
                        [1.0, 0.0, 0.0]
                    } else {
                        [0.0, 1.0, 0.0]
                    }
                } else {
                    // TASK: Interpolate between red and green, depending on
                    //       (signed) distance.
                    [1.0, 1.0, 0.0]
                };

                let alpha = if vertex.distance.abs() <= threshold_a.0 {
                    threshold_a.1
                } else if vertex.distance.abs() <= threshold_b.0 {
                    (vertex.distance.abs() - threshold_a.0)
                        / (threshold_b.0 - threshold_a.0)
                        * (threshold_a.1 - threshold_b.1)
                        + threshold_b.1
                } else {
                    threshold_b.1
                };

                Vertex {
                    position: vertex.point.into(),
                    normal: [0.0, 0.0, 0.0], // normal not used for grid
                    color: [color[0], color[1], color[2], alpha],
                }
            })
            .collect();

        Self { vertices, indices }
    }
}

impl From<mesh::Mesh> for Vertices {
    fn from(mesh: mesh::Mesh) -> Self {
        let vertices: Vec<_> = mesh.vertices().collect();

        let mut indices_by_vertex_with_normal = IndexMap::new();
        let mut indices = Vec::new();

        for [i0, i1, i2] in mesh.triangles() {
            let v0 = vertices[i0 as usize];
            let v1 = vertices[i1 as usize];
            let v2 = vertices[i2 as usize];

            let normal = (v1 - v0).cross(&(v2 - v0)).normalize();

            let v0 = [R32::from(v0.x), R32::from(v0.y), R32::from(v0.z)];
            let v1 = [R32::from(v1.x), R32::from(v1.y), R32::from(v1.z)];
            let v2 = [R32::from(v2.x), R32::from(v2.y), R32::from(v2.z)];

            let normal = [
                R32::from(normal.x),
                R32::from(normal.y),
                R32::from(normal.z),
            ];

            let next_index = indices_by_vertex_with_normal.len();
            let i0 = *indices_by_vertex_with_normal
                .entry((v0, normal))
                .or_insert(next_index);

            let next_index = indices_by_vertex_with_normal.len();
            let i1 = *indices_by_vertex_with_normal
                .entry((v1, normal))
                .or_insert(next_index);

            let next_index = indices_by_vertex_with_normal.len();
            let i2 = *indices_by_vertex_with_normal
                .entry((v2, normal))
                .or_insert(next_index);

            indices.push(i0 as u32);
            indices.push(i1 as u32);
            indices.push(i2 as u32);
        }

        let vertices = indices_by_vertex_with_normal
            .keys()
            .map(|(vertex, normal)| Vertex {
                position: [
                    vertex[0].into_inner(),
                    vertex[1].into_inner(),
                    vertex[2].into_inner(),
                ],
                normal: [
                    normal[0].into_inner(),
                    normal[1].into_inner(),
                    normal[2].into_inner(),
                ],
                color: [1.0, 0.0, 0.0, 1.0],
            })
            .collect();

        Self { vertices, indices }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
