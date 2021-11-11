use crate::mesh::{Mesh, MeshMaker};

pub fn shape_to_triangle_mesh(shape: &fj::Shape) -> Mesh {
    match shape {
        fj::Shape::Cube(cube) => cube_to_triangle_mesh(cube),
    }
}

fn cube_to_triangle_mesh(cube: &fj::Cube) -> Mesh {
    let mut mesh = MeshMaker::new();
    let s = cube.size / 2.;

    // Define a cube
    let v0 = [-s, -s, -s];
    let v1 = [-s, -s, s];
    let v2 = [-s, s, -s];
    let v3 = [-s, s, s];
    let v4 = [s, -s, -s];
    let v5 = [s, -s, s];
    let v6 = [s, s, -s];
    let v7 = [s, s, s];

    // left
    mesh.triangle([v0, v1, v2]);
    mesh.triangle([v2, v1, v3]);

    // right
    mesh.triangle([v4, v6, v5]);
    mesh.triangle([v6, v7, v5]);

    // front
    mesh.triangle([v0, v4, v1]);
    mesh.triangle([v4, v5, v1]);

    // back
    mesh.triangle([v2, v3, v6]);
    mesh.triangle([v6, v3, v7]);

    // bottom
    mesh.triangle([v0, v2, v6]);
    mesh.triangle([v0, v6, v4]);

    // top
    mesh.triangle([v1, v5, v7]);
    mesh.triangle([v1, v7, v3]);

    mesh.make()
}
