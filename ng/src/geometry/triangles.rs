use crate::{geometry::vertices::Vertices as _, math::Point};

/// The triangles that make up a shape
pub trait Triangles {
    /// Compute the triangles of a shape
    fn triangles(&self) -> Vec<Triangle>;
}

pub type Triangle = [Point; 3];

impl Triangles for fj::Shape {
    fn triangles(&self) -> Vec<Triangle> {
        match self {
            Self::Shape2d(shape) => shape.triangles(),
            Self::Shape3d(shape) => shape.triangles(),
        }
    }
}

impl Triangles for fj::Shape2d {
    fn triangles(&self) -> Vec<Triangle> {
        match self {
            Self::Square(shape) => shape.triangles(),
        }
    }
}

impl Triangles for fj::Shape3d {
    fn triangles(&self) -> Vec<Triangle> {
        match self {
            Self::Cube(shape) => shape.triangles(),
            Self::Sweep(shape) => shape.triangles(),
        }
    }
}

impl Triangles for fj::Square {
    fn triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]]);
        triangles.push([v[0], v[2], v[3]]);

        triangles
    }
}

impl Triangles for fj::Cube {
    fn triangles(&self) -> Vec<Triangle> {
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

impl Triangles for fj::Sweep {
    fn triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        // PARTIAL IMPLEMENTATION
        //
        // The top face is currently missing.
        // TASK: Add top face.

        // Bottom face
        // TASK: This shows the wrong side on the outside. The triangles need to
        //       be inverted.
        triangles.extend(self.shape.triangles());

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
