use crate::{
    geometry::attributes::{BoundingVolume, Geometry},
    mesh::Mesh,
};

use super::{grid, Grid};

pub fn to_mesh(
    geometry: &(impl Geometry<3> + BoundingVolume<3>),
    resolution: f32,
) -> Mesh {
    let grid_descriptor = grid::Descriptor {
        aabb: geometry.aabb(),
        resolution,
    };
    let grid = Grid::from_descriptor(grid_descriptor, geometry);

    let mut mesh = Mesh::new();

    for edge in grid.edges_at_surface() {
        let [a, b, c, d] = grid.neighbors_of_edge(edge);

        mesh.triangle(a, b, d);
        mesh.triangle(b, c, d);
    }

    mesh
}
