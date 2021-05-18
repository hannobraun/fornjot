use crate::{
    geometry::attributes::{BoundingVolume, Distance},
    mesh::Mesh,
};

use super::{Grid, GridDescriptor};

pub fn to_mesh(
    isosurface: &(impl Distance<3> + BoundingVolume<3>),
    resolution: f32,
) -> Mesh {
    let grid_descriptor = GridDescriptor {
        aabb: isosurface.aabb(),
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
