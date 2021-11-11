use crate::{
    geometry::vertices::Vertices as _,
    mesh::{Mesh, MeshMaker},
};

/// Convert a shape into a [`Mesh`]
pub trait ToMesh {
    /// Convert a shape into a [`Mesh`]
    fn to_mesh(&self) -> Mesh;
}

impl ToMesh for fj::Shape {
    fn to_mesh(&self) -> Mesh {
        match self {
            fj::Shape::Cube(cube) => cube.to_mesh(),
        }
    }
}

impl ToMesh for fj::Cube {
    fn to_mesh(&self) -> Mesh {
        let mut mesh = MeshMaker::new();

        let v = self.vertices();

        // left
        mesh.triangle([v[0], v[1], v[2]]);
        mesh.triangle([v[2], v[1], v[3]]);

        // right
        mesh.triangle([v[4], v[6], v[5]]);
        mesh.triangle([v[6], v[7], v[5]]);

        // front
        mesh.triangle([v[0], v[4], v[1]]);
        mesh.triangle([v[4], v[5], v[1]]);

        // back
        mesh.triangle([v[2], v[3], v[6]]);
        mesh.triangle([v[6], v[3], v[7]]);

        // bottom
        mesh.triangle([v[0], v[2], v[6]]);
        mesh.triangle([v[0], v[6], v[4]]);

        // top
        mesh.triangle([v[1], v[5], v[7]]);
        mesh.triangle([v[1], v[7], v[3]]);

        mesh.make()
    }
}
