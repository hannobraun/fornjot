use nalgebra::Point;

use crate::geometry::{
    aabb::Aabb,
    attributes::{BoundingVolume, Geometry, Sample},
};

pub struct Difference<A, B> {
    pub a: A,
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
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Sample<D> {
        let point = point.into();

        let sample_a = self.a.sample(point);
        let sample_b = self.b.sample(point);

        let sample_b = Sample {
            distance: -sample_b.distance,
            normal: -sample_b.normal,
            ..sample_b
        };

        if sample_a.distance > sample_b.distance {
            sample_a
        } else {
            sample_b
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{SVector, Unit};

    use crate::geometry::{attributes::Geometry as _, shapes::Sphere};

    use super::Difference;

    #[test]
    fn distance() {
        let difference = Difference {
            a: Sphere::new().with_radius(1.0),
            b: Sphere::new().with_radius(0.5),
        };

        assert_eq!(difference.sample([0.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(difference.sample([0.5, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.sample([0.625, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.sample([0.75, 0.0, 0.0]).distance, -0.25);
        assert_eq!(difference.sample([0.875, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.sample([1.0, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.sample([1.5, 0.0, 0.0]).distance, 0.5);
    }

    #[test]
    fn normal() {
        let difference = Difference {
            a: Sphere::new().with_radius(1.0),
            b: Sphere::new().with_radius(0.5),
        };

        let values = [
            ([0.25, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            ([0.5, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            ([0.625, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            ([0.875, 0.0, 0.0], [1.0, 0.0, 0.0]),
            ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            ([1.5, 0.0, 0.0], [1.0, 0.0, 0.0]),
        ];

        for (actual, expected) in values {
            println!("point: {:?}", actual);
            assert_eq!(
                difference.sample(actual).normal,
                Unit::new_normalize(SVector::<_, 3>::from(expected))
            );
        }
    }
}
