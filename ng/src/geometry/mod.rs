pub mod bounding_volume;
pub mod edges;
pub mod faces;
pub mod shapes;
pub mod vertices;

use self::bounding_volume::Aabb;

/// Implemented by all shapes
pub trait Shape {
    /// Compute the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn aabb(&self) -> Aabb;
}

macro_rules! dispatch_shape {
    ($method:ident($($arg_name:ident: $arg_ty:ident)*)) => {
        fn $method(&self, $($arg_name: $arg_ty)*) -> Aabb {
            match self {
                Self::Shape2d(shape) => shape.$method($($arg_name)*),
                Self::Shape3d(shape) => shape.$method($($arg_name)*),
            }
        }
    };
}

macro_rules! dispatch_shape2d {
    ($method:ident($($arg_name:ident: $arg_ty:ident)*)) => {
        fn $method(&self) -> Aabb {
            match self {
                Self::Circle(shape) => shape.$method($($arg_name)*),
                Self::Difference(shape) => shape.$method($($arg_name)*),
                Self::Square(shape) => shape.$method($($arg_name)*),
            }
        }
    };
}

macro_rules! dispatch_shape3d {
    ($method:ident($($arg_name:ident: $arg_ty:ident)*)) => {
        fn $method(&self) -> Aabb {
            match self {
                Self::Sweep(shape) => shape.$method($($arg_name)*),
            }
        }
    };
}

impl Shape for fj::Shape {
    dispatch_shape!(aabb());
}

impl Shape for fj::Shape2d {
    dispatch_shape2d!(aabb());
}

impl Shape for fj::Shape3d {
    dispatch_shape3d!(aabb());
}
