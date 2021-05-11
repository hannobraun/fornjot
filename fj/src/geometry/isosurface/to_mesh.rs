use crate::{
    geometry::attributes::{BoundingVolume, Distance},
    mesh::Mesh,
};

use super::{Grid, GridDescriptor};

pub fn to_mesh(
    isosurface: &(impl Distance + BoundingVolume),
    resolution: f32,
) -> Mesh {
    let aabb = isosurface.aabb();
    let grid_descriptor = GridDescriptor {
        min: aabb.min,
        max: aabb.max,
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
