use nalgebra::vector;

use crate::{
    geometry::{edges::Edges as _, vertices::Vertices as _},
    math::{Point, Vector},
};

/// Access a shape's faces
pub trait Faces {
    /// Compute triangles to approximate the shape's faces
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn triangles(&self, tolerance: f32) -> Triangles;
}

/// The triangles that approximate a shape's faces
pub struct Triangles(pub Vec<Triangle>);

impl Triangles {
    /// Create new instance of `Triangles`
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a triangle
    pub fn push(&mut self, triangle: impl Into<Triangle>) {
        self.0.push(triangle.into())
    }
}

/// A triangle
///
/// Or more specifically, three points. Currently no validation is done to
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

impl Faces for fj::Shape {
    fn triangles(&self, tolerance: f32) -> Triangles {
        match self {
            Self::Shape2d(shape) => shape.triangles(tolerance),
            Self::Shape3d(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape2d {
    fn triangles(&self, tolerance: f32) -> Triangles {
        match self {
            Self::Circle(shape) => shape.triangles(tolerance),
            Self::Square(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape3d {
    fn triangles(&self, tolerance: f32) -> Triangles {
        match self {
            Self::Sweep(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Circle {
    fn triangles(&self, tolerance: f32) -> Triangles {
        dbg!(self.segments(tolerance));

        // TASK: Implement.
        todo!()
    }
}

impl Faces for fj::Square {
    fn triangles(&self, _: f32) -> Triangles {
        let mut triangles = Triangles::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]]);
        triangles.push([v[0], v[2], v[3]]);

        triangles
    }
}

impl Faces for fj::Sweep {
    fn triangles(&self, tolerance: f32) -> Triangles {
        let mut triangles = Triangles::new();

        let original_triangles = self.shape.triangles(tolerance);

        // Bottom face
        triangles.0.extend(
            original_triangles
                .0
                .iter()
                .map(|triangle| triangle.invert()),
        );

        // Top face
        triangles
            .0
            .extend(original_triangles.0.iter().map(|triangle| {
                triangle.translate(vector![0.0, 0.0, self.length])
            }));

        let segments = self.shape.segments(tolerance);

        let mut quads = Vec::new();
        for segment in segments.0 {
            let [v0, v1] = segment.0;
            let [v3, v2] = segment.translate(vector![0.0, 0.0, self.length]).0;

            quads.push([v0, v1, v2, v3]);
        }

        for [v0, v1, v2, v3] in quads {
            triangles.push([v0, v1, v2]);
            triangles.push([v0, v2, v3]);
        }

        triangles
    }
}
