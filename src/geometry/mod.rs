pub mod bounding_volume;
pub mod edges;
pub mod faces;
pub mod shapes;

use crate::math::Point;

use self::{bounding_volume::Aabb, edges::Edges, faces::Triangle};

/// Implemented by all shapes
pub trait Shape {
    /// Access the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn bounding_volume(&self) -> Aabb;

    /// Compute triangles to approximate the shape's faces
    ///
    /// The shape defined by the approximated triangles must be fully contained
    /// within the actual shape.
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn triangles(&self, tolerance: f64) -> Vec<Triangle>;

    /// Access the edges of the shape
    fn edges(&self) -> Edges;

    /// Return the shape's vertices
    fn vertices(&self) -> Vec<Point>;
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
    bounding_volume() -> Aabb;
    triangles(tolerance: f64) -> Vec<Triangle>;
    edges() -> Edges;
    vertices() -> Vec<Point>;
}
