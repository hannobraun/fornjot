use crate::{
    geometry::attributes::{BoundingVolume, Distance},
    mesh::Mesh,
};

use super::{Grid, GridDescriptor};

pub fn to_mesh<Sdf>(isosurface: Sdf, resolution: f32) -> Mesh
where
    Sdf: Distance + BoundingVolume,
{
    let aabb = isosurface.aabb();
    let grid_descriptor = GridDescriptor {
        min: aabb.mins,
        max: aabb.maxs,
        resolution,
    };
    let grid = Grid::from_descriptor(grid_descriptor, isosurface);

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
