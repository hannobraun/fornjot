pub mod primitives;

use nalgebra::Point2;

pub trait Boundary {
    fn boundary(&self, s: f32) -> Point2<f32>;
}
