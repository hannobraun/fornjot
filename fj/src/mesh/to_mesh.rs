use crate::geometry::{
    attributes::{BoundingVolume, Distance},
    isosurface::{self, Grid},
};

use super::Mesh;

pub trait ToMesh {
    fn to_mesh(self, resolution: f32) -> Mesh;
}

impl ToMesh for Mesh {
    fn to_mesh(self, _: f32) -> Mesh {
        self
    }
}

impl<T> ToMesh for T
where
    T: BoundingVolume + Distance,
{
    // TASK: Extract code into new function in `isosurface`.
    fn to_mesh(self, resolution: f32) -> Mesh {
        let aabb = self.aabb();
        let grid_descriptor = isosurface::GridDescriptor {
            min: aabb.mins,
            max: aabb.maxs,
            resolution,
        };
        let grid = Grid::from_descriptor(grid_descriptor, self);

        let mut mesh = Mesh::new();

        for edge in grid.edges() {
            if edge.at_surface() {
                let [a, b, c, d] = grid.neighbors_of_edge(edge);

                mesh.triangle(a, b, d);
                mesh.triangle(b, c, d);
            }
        }

        mesh
    }
}
