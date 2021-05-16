use nalgebra::Point;

use crate::geometry::attributes::Distance;

pub struct Difference<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B, const D: usize> Distance<D> for Difference<A, B>
where
    A: Distance<D>,
    B: Distance<D>,
{
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32 {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        f32::max(dist_a, -dist_b)
    }
}
