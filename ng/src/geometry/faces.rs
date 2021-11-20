use nalgebra::vector;

use crate::{
    geometry::{edges::Edges as _, vertices::Vertices as _},
    math::{Point, Vector},
};

/// Access a shape's faces
pub trait Faces {
    /// Compute triangles to approximate the shape's faces
    ///
    /// The shape defined by the approximated triangles must be fully contained
    /// within the actual shape.
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn triangles(&self, tolerance: f64) -> Triangles;
}

/// The triangles that approximate a shape's faces
pub type Triangles = Vec<Triangle>;

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
    fn triangles(&self, tolerance: f64) -> Triangles {
        match self {
            Self::Shape2d(shape) => shape.triangles(tolerance),
            Self::Shape3d(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape2d {
    fn triangles(&self, tolerance: f64) -> Triangles {
        match self {
            Self::Circle(shape) => shape.triangles(tolerance),
            Self::Difference(shape) => shape.triangles(tolerance),
            Self::Square(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape3d {
    fn triangles(&self, tolerance: f64) -> Triangles {
        match self {
            Self::Sweep(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Circle {
    fn triangles(&self, tolerance: f64) -> Triangles {
        let vertices: Vec<_> = self
            .edge_vertices(tolerance)
            .into_iter()
            .flatten()
            .collect();
        triangulate(&vertices)
    }
}

impl Faces for fj::Difference {
    fn triangles(&self, tolerance: f64) -> Triangles {
        // TASK: Carefully think about the limits of this algorithm, and make
        //       sure to panic with a `todo!` in cases that are not supported.

        let a: Vec<_> = self
            .a
            .edge_vertices(tolerance)
            .into_iter()
            .flatten()
            .collect();
        let b: Vec<_> = self
            .b
            .edge_vertices(tolerance)
            .into_iter()
            .flatten()
            .collect();

        let mut vertices = Vec::new();
        vertices.extend(&a);
        vertices.extend(&b);

        let mut triangles = triangulate(&vertices);

        // Now we have a full Delaunay triangulation of all vertices. We still
        // need to filter out the triangles that aren't actually part of the
        // difference.
        triangles.retain(|triangle| {
            let mut edges_of_b = 0;

            for [v0, v1] in triangle.edges() {
                if b.contains(&v0) && b.contains(&v1) {
                    edges_of_b += 1;
                }
            }

            edges_of_b <= 1
        });

        triangles
    }
}

impl Faces for fj::Square {
    fn triangles(&self, _: f64) -> Triangles {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

        triangles
    }
}

impl Faces for fj::Sweep {
    fn triangles(&self, tolerance: f64) -> Triangles {
        let mut triangles = Vec::new();

        let original_triangles = self.shape.triangles(tolerance);

        // Bottom face
        triangles.extend(
            original_triangles.iter().map(|triangle| triangle.invert()),
        );

        // Top face
        triangles.extend(original_triangles.iter().map(|triangle| {
            triangle.translate(vector![0.0, 0.0, self.length])
        }));

        let segments = self.shape.edge_segments(tolerance);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = segment.0;
            let [v3, v2] = segment.translate(vector![0.0, 0.0, self.length]).0;

            quads.push([v0, v1, v2, v3]);
        }

        for [v0, v1, v2, v3] in quads {
            triangles.push([v0, v1, v2].into());
            triangles.push([v0, v2, v3].into());
        }

        triangles
    }
}

fn triangulate(vertices: &[Point]) -> Triangles {
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
