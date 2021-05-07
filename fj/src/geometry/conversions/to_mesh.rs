use std::convert::Infallible;

use crate::geometry::{
    attributes::{BoundingVolume, Distance},
    isosurface::{self, Grid},
    shapes::Mesh,
};

pub trait ToMesh {
    type Error;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error>;
}

impl ToMesh for Mesh {
    type Error = Infallible;

    fn to_mesh(self, _tolerance: f32) -> Result<Mesh, Self::Error> {
        Ok(self)
    }
}

impl<T> ToMesh for T
where
    T: BoundingVolume + Distance,
{
    type Error = Infallible;

    fn to_mesh(self, resolution: f32) -> Result<Mesh, Self::Error> {
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

                // TASK: Make sure that triangles face the right direction.
                mesh.triangle(a, b, d);
                mesh.triangle(b, c, d);
            }
        }

        Ok(mesh)
    }
}
