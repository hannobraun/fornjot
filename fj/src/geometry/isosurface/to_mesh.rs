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
        // TASK: Remove `at_surface` check, once it becomes redundant.
        // TASK: This check filters out some of the wrong edges. I think the
        //       problem is floating point precision. Edges whose length should
        //       be equal to the resolution are in fact slightly longer.
        if edge.at_surface(resolution) {
            let [a, b, c, d] = grid.neighbors_of_edge(edge);

            mesh.triangle(a, b, d);
            mesh.triangle(b, c, d);
        }
    }

    mesh
}
