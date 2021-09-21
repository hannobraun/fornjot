use crate::geometry::attributes::{BoundingVolume, Mesh, SignedDistanceField};

use super::{grid, Grid};

/// Convert functionally defined geometry into a triangle mesh
pub fn to_mesh(
    geometry: &(impl SignedDistanceField<3> + BoundingVolume<3>),
    resolution: f32,
) -> (Mesh<3>, Grid) {
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

    (mesh, grid)
}
