pub mod bounding_volume;
pub mod edges;
pub mod faces;
pub mod shapes;
pub mod vertices;

use crate::math::Point;

use self::{bounding_volume::Aabb, edges::Segment};

/// Implemented by all shapes
pub trait Shape {
    /// Compute the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn aabb(&self) -> Aabb;

    /// Compute vertices to approximate the shape's edges
    ///
    /// Returns a `Vec` that contains a `Vec<Point>` for each edge of the shape.
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edges of the shape.
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>>;

    /// Compute line segments to approximate the shape's edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn edge_segments(&self, tolerance: f64) -> Vec<Segment> {
        let mut segments = Vec::new();
        let edges = self.edge_vertices(tolerance);

        for mut vertices in edges {
            // We're about to convert these vertices into line segments, and we
            // need a connection from the last to the first.
            match vertices.first() {
                Some(&vertex) => vertices.push(vertex),
                None => {
                    // If there is not first vertex, there are no vertices. If
                    // there are no vertices, there are no segments.
                    return segments;
                }
            }

            for segment in vertices.windows(2) {
                let v0 = segment[0];
                let v1 = segment[1];

                segments.push([v0, v1].into());
            }
        }

        segments
    }
}

macro_rules! dispatch {
    ($($method:ident($($arg_name:ident: $arg_ty:ident)*) -> $ret:ty;)*) => {
        impl Shape for fj::Shape {
            $(
                fn $method(&self, $($arg_name: $arg_ty)*) -> $ret {
                    match self {
                        Self::Shape2d(shape) => shape.$method($($arg_name)*),
                        Self::Shape3d(shape) => shape.$method($($arg_name)*),
                    }
                }
            )*
        }

        impl Shape for fj::Shape2d {
            $(
                fn $method(&self, $($arg_name: $arg_ty)*) -> $ret {
                    match self {
                        Self::Circle(shape) => shape.$method($($arg_name)*),
                        Self::Difference(shape) => shape.$method($($arg_name)*),
                        Self::Square(shape) => shape.$method($($arg_name)*),
                    }
                }
            )*
        }

        impl Shape for fj::Shape3d {
            $(
                fn $method(&self, $($arg_name: $arg_ty)*) -> $ret {
                    match self {
                        Self::Sweep(shape) => shape.$method($($arg_name)*),
                    }
                }
            )*
        }
    };
}

dispatch! {
    aabb() -> Aabb;
    edge_vertices(tolerance: f64) -> Vec<Vec<Point>>;
    edge_segments(tolerance: f64) -> Vec<Segment>;
}
