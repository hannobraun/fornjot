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
            Self::Shape2d(shape) => shape.to_mesh(),
            Self::Shape3d(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Shape2d {
    fn to_mesh(&self) -> Mesh {
        match self {
            Self::Square(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Shape3d {
    fn to_mesh(&self) -> Mesh {
        match self {
            Self::Cube(shape) => shape.to_mesh(),
            Self::Sweep(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Square {
    fn to_mesh(&self) -> Mesh {
        let mut mesh = MeshMaker::new();

        let v = self.vertices();

        mesh.triangle([v[0], v[1], v[2]]);
        mesh.triangle([v[0], v[2], v[3]]);

        mesh.make()
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

impl ToMesh for fj::Sweep {
    fn to_mesh(&self) -> Mesh {
        let mut mesh = MeshMaker::new();

        // PARTIAL IMPLEMENTATION
        //
        // Only the side faces are being generated. Bottom and top faces are
        // currently missing.
        // TASK: Add bottom face.
        // TASK: Add top face.

        // In the next step, we're going to collect those pairs of vertices into
        // quads. But we also need to make sure we'll get the last quad, which
        // is made up of the last and first pair.
        let mut vertex_pairs = self.vertices().vertex_pairs();
        vertex_pairs.push(vertex_pairs[0]);

        // Collect all vertices that make up the quads of the side faces.
        //
        // This can be simplified (and made non-panicky), once `array_windows`
        // is stabilized.
        let quads = vertex_pairs.windows(2).map(|window| {
            let [v0, v3] = window[0];
            let [v1, v2] = window[1];

            [v0, v1, v2, v3]
        });

        for [v0, v1, v2, v3] in quads {
            mesh.triangle([v0, v1, v2]);
            mesh.triangle([v0, v2, v3]);
        }

        mesh.make()
    }
}
