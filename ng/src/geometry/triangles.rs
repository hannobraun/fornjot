use nalgebra::vector;

use crate::{
    geometry::vertices::Vertices as _,
    math::{Point, Vector},
};

/// The triangles that make up a shape
pub trait Triangles {
    /// Compute the triangles of a shape
    fn triangles(&self) -> Vec<Triangle>;
}

/// A triangle
///
/// Or more specifically, three points. Currently now validation is done, to
/// ensure those points form an actual triangle.
#[derive(Clone, Copy)]
pub struct Triangle(pub [Point; 3]);

impl Triangle {
    /// Invert the triangle
    ///
    /// Inverts the order of triangle vertices.
    pub fn invert(self) -> Self {
        let [v0, v1, v2] = self.0;
        Self([v0, v2, v1])
    }

    /// Translate the triangle
    ///
    /// Translate all triangle vertices by the given vector.
    pub fn translate(self, vector: Vector) -> Self {
        let vertices = self.0.map(|vertex| vertex + vector);
        Self(vertices)
    }
}

impl From<[Point; 3]> for Triangle {
    fn from(vertices: [Point; 3]) -> Self {
        Self(vertices)
    }
}

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
            Self::Circle(_) => {
                // TASK: Implement.
                todo!()
            }
            Self::Square(shape) => shape.triangles(),
        }
    }
}

impl Triangles for fj::Shape3d {
    fn triangles(&self) -> Vec<Triangle> {
        match self {
            Self::Sweep(shape) => shape.triangles(),
        }
    }
}

impl Triangles for fj::Square {
    fn triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

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

        let original_triangles = self.shape.triangles();

        // Bottom face
        triangles.extend(
            original_triangles.iter().map(|triangle| triangle.invert()),
        );

        // Top face
        triangles.extend(original_triangles.iter().map(|triangle| {
            triangle.translate(vector![0.0, 0.0, self.length])
        }));

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
            triangles.push([v0, v1, v2].into());
            triangles.push([v0, v2, v3].into());
        }

        triangles
    }
}
