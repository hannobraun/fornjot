use nalgebra::Point;

use crate::geometry::{
    aabb::Aabb,
    traits::{BoundingVolume, Distance, Geometry},
};

/// The difference of two bodies
pub struct Difference<A, B> {
    /// The body that is being subtracted from
    pub a: A,

    /// The body that is being subtracted
    pub b: B,
}

impl<A, B, const D: usize> BoundingVolume<D> for Difference<A, B>
where
    A: BoundingVolume<D>,
{
    fn aabb(&self) -> Aabb<D> {
        // Since `self.b` is subtracted from `self.a`, the bounding volume of
        // the difference is not going to be bigger than that of `self.a`. Just
        // taking the bounding volume from `self.a` is certainly not optimal,
        // but good enough for now.
        self.a.aabb()
    }
}

impl<A, B, const D: usize> Geometry<D> for Difference<A, B>
where
    A: Geometry<D>,
    B: Geometry<D>,
{
    fn distance(&self, point: impl Into<Point<f32, D>>) -> Distance<D> {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        let dist_b = Distance {
            distance: -dist_b.distance,
            ..dist_b
        };

        if dist_a.distance > dist_b.distance {
            dist_a
        } else {
            dist_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{shapes::Sphere, traits::Geometry as _};

    use super::Difference;

    #[test]
    fn distance() {
        let difference = Difference {
            a: Sphere::new().with_radius(1.0),
            b: Sphere::new().with_radius(0.5),
        };

        assert_eq!(difference.distance([0.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(difference.distance([0.5, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.distance([0.625, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.distance([0.75, 0.0, 0.0]).distance, -0.25);
        assert_eq!(difference.distance([0.875, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.distance([1.0, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.distance([1.5, 0.0, 0.0]).distance, 0.5);
    }
}
