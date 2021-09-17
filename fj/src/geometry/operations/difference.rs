/// The difference of two bodies
pub struct Difference<A, B> {
    /// The body that is being subtracted from
    pub a: A,

    /// The body that is being subtracted
    pub b: B,
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        attributes::SignedDistanceField as _, shapes::Sphere,
    };

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
