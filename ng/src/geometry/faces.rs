use crate::math::{Point, Vector};

/// Access a shape's faces
pub trait Faces {
    /// Compute triangles to approximate the shape's faces
    ///
    /// The shape defined by the approximated triangles must be fully contained
    /// within the actual shape.
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn triangles(&self, tolerance: f64) -> Vec<Triangle>;
}

/// A triangle
///
/// Or more specifically, three points. Currently no validation is done to
/// ensure those points form an actual triangle.
#[derive(Clone, Copy, Debug)]
pub struct Triangle(pub [Point; 3]);

impl Triangle {
    /// Access the edges of the triangle
    pub fn edges(&self) -> impl Iterator<Item = [Point; 2]> {
        let v0 = self.0[0];
        let v1 = self.0[1];
        let v2 = self.0[2];

        [[v0, v1], [v1, v2], [v2, v0]].into_iter()
    }

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
    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        match self {
            Self::Shape2d(shape) => shape.triangles(tolerance),
            Self::Shape3d(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape2d {
    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        match self {
            Self::Circle(shape) => shape.triangles(tolerance),
            Self::Difference(shape) => shape.triangles(tolerance),
            Self::Square(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape3d {
    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        match self {
            Self::Sweep(shape) => shape.triangles(tolerance),
        }
    }
}

pub fn triangulate(vertices: &[Point]) -> Vec<Triangle> {
    let points: Vec<_> = vertices
        .iter()
        .map(|vertex| delaunator::Point {
            x: vertex.x,
            y: vertex.y,
        })
        .collect();

    let triangulation = delaunator::triangulate(&points);

    let mut triangles = Vec::new();
    for triangle in triangulation.triangles.chunks(3) {
        let i0 = triangle[0];
        let i1 = triangle[1];
        let i2 = triangle[2];

        triangles.push([vertices[i0], vertices[i2], vertices[i1]].into());
    }

    triangles
}
