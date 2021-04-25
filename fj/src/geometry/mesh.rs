use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Vector3;

use crate::{
    geometry::{shapes::Point, Triangle3, Triangles},
    graphics,
};

pub struct Mesh {
    indices_by_position: HashMap<Point<3>, Index>,
    positions_by_index: HashMap<Index, Point<3>>,
    indices_by_vertex: HashMap<Vertex, graphics::Index>,

    vertices: Vec<Vertex>,
    indices: Vec<graphics::Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            indices_by_position: HashMap::new(),
            positions_by_index: HashMap::new(),
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertex(&mut self, vertex: impl Into<Point<3>>) -> Index {
        let position = vertex.into();

        let new_index =
            Index(self.indices_by_position.len().try_into().unwrap());
        let index = *self
            .indices_by_position
            .entry(position)
            .or_insert(new_index);
        self.positions_by_index.insert(index, position);

        index
    }

    pub fn triangle(&mut self, i0: Index, i1: Index, i2: Index) {
        let p0: nalgebra::Point<f32, 3> = self.positions_by_index[&i0].into();
        let p1: nalgebra::Point<f32, 3> = self.positions_by_index[&i1].into();
        let p2: nalgebra::Point<f32, 3> = self.positions_by_index[&i2].into();

        let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
        let normal = normal.map(|v| R32::from_inner(v));

        let v0 = Vertex {
            position: p0.into(),
            normal,
        };
        let v1 = Vertex {
            position: p1.into(),
            normal,
        };
        let v2 = Vertex {
            position: p2.into(),
            normal,
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[graphics::Index] {
        self.indices.as_slice()
    }

    pub fn into_graphics_mesh(self) -> graphics::Mesh {
        let vertices = self
            .vertices
            .into_iter()
            .map(|vertex| graphics::Vertex {
                position: [
                    vertex.position[0].into_inner(),
                    vertex.position[1].into_inner(),
                    vertex.position[2].into_inner(),
                ],
                normal: [
                    vertex.normal[0].into_inner(),
                    vertex.normal[1].into_inner(),
                    vertex.normal[2].into_inner(),
                ],
            })
            .collect();
        let indices = self.indices;

        graphics::Mesh { vertices, indices }
    }

    pub fn triangles(&self) -> Triangles {
        let mut indices = self.indices().iter();

        let mut next_triangle = || {
            let &i0 = indices.next()?;
            let &i1 = indices.next()?;
            let &i2 = indices.next()?;

            let v0 = self.vertices[i0 as usize].position;
            let v1 = self.vertices[i1 as usize].position;
            let v2 = self.vertices[i2 as usize].position;

            Some(Triangle3::new(
                [v0[0].into_inner(), v0[1].into_inner(), v0[2].into_inner()],
                [v1[0].into_inner(), v1[1].into_inner(), v1[2].into_inner()],
                [v2[0].into_inner(), v2[1].into_inner(), v2[2].into_inner()],
            ))
        };

        let mut triangles = Vec::new();

        while let Some(triangle) = next_triangle() {
            triangles.push(triangle);
        }

        Triangles(triangles)
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> graphics::Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub position: Point<3>,
    pub normal: Vector3<R32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Index(u16);

#[cfg(test)]
mod tests {
    use decorum::R32;
    use nalgebra::{Point3, Vector3};

    use crate::geometry::Triangle3;

    use super::{Mesh, Vertex};

    #[test]
    fn vertex_should_return_same_index_for_same_vertex() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [0.5, 0.0, 0.0];
        let v2 = [0.0, 0.5, 0.0];

        let i0 = mesh.vertex(v0);
        let i1 = mesh.vertex(v1);
        let i2 = mesh.vertex(v2);

        assert_eq!(i0, mesh.vertex(v0));
        assert_eq!(i1, mesh.vertex(v1));
        assert_eq!(i2, mesh.vertex(v2));
    }

    #[test]
    fn mesh_should_convert_triangle_into_vertices_and_indices() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [0.5, 0.0, 0.0];
        let v2 = [0.0, 0.5, 0.0];

        let i0 = mesh.vertex(v0);
        let i1 = mesh.vertex(v1);
        let i2 = mesh.vertex(v2);

        mesh.triangle(i0, i1, i2);

        let mut vertices = Vec::new();
        for &i in mesh.indices() {
            vertices.push(mesh.vertices()[i as usize]);
        }

        let normal = Vector3::new(
            R32::from_inner(0.0),
            R32::from_inner(0.0),
            R32::from_inner(1.0),
        );
        assert_eq!(
            vertices,
            vec![
                Vertex {
                    position: Point3::from(
                        Point3::from(v0).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v1).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v2).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
            ]
        );
    }

    #[test]
    fn mesh_should_return_triangles() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let i0 = mesh.vertex(v0);
        let i1 = mesh.vertex(v1);
        let i2 = mesh.vertex(v2);

        mesh.triangle(i0, i1, i2);
        mesh.triangle(i0, i2, i1);

        let triangles = mesh.triangles();
        assert_eq!(
            triangles.0,
            vec![Triangle3::new(v0, v1, v2), Triangle3::new(v0, v2, v1)]
        );
    }

    // TASK: Add method that inverts triangles of a mesh.
    // TASK: Add method that merges another mesh into the mesh.
}
