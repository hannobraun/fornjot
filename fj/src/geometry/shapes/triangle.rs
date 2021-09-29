use crate::math::Point;

pub struct Triangle<const D: usize>(pub [Point<D>; 3]);
