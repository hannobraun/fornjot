use crate::{geometry::vertices::Vertices as _, math::Point};

/// Convert a shape into a [`Mesh`]
pub trait ToMesh {
    /// Convert a shape into a [`Mesh`]
    fn to_mesh(&self) -> Vec<Triangle>;
}

pub type Triangle = [Point; 3];

impl ToMesh for fj::Shape {
    fn to_mesh(&self) -> Vec<Triangle> {
        match self {
            Self::Shape2d(shape) => shape.to_mesh(),
            Self::Shape3d(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Shape2d {
    fn to_mesh(&self) -> Vec<Triangle> {
        match self {
            Self::Square(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Shape3d {
    fn to_mesh(&self) -> Vec<Triangle> {
        match self {
            Self::Cube(shape) => shape.to_mesh(),
            Self::Sweep(shape) => shape.to_mesh(),
        }
    }
}

impl ToMesh for fj::Square {
    fn to_mesh(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]]);
        triangles.push([v[0], v[2], v[3]]);

        triangles
    }
}

impl ToMesh for fj::Cube {
    fn to_mesh(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        // left
        triangles.push([v[0], v[1], v[2]]);
        triangles.push([v[2], v[1], v[3]]);

        // right
        triangles.push([v[4], v[6], v[5]]);
        triangles.push([v[6], v[7], v[5]]);

        // front
        triangles.push([v[0], v[4], v[1]]);
        triangles.push([v[4], v[5], v[1]]);

        // back
        triangles.push([v[2], v[3], v[6]]);
        triangles.push([v[6], v[3], v[7]]);

        // bottom
        triangles.push([v[0], v[2], v[6]]);
        triangles.push([v[0], v[6], v[4]]);

        // top
        triangles.push([v[1], v[5], v[7]]);
        triangles.push([v[1], v[7], v[3]]);

        triangles
    }
}

impl ToMesh for fj::Sweep {
    fn to_mesh(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

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
            triangles.push([v0, v1, v2]);
            triangles.push([v0, v2, v3]);
        }

        triangles
    }
}
