use nalgebra::Point3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub a: Point3<f32>,
    pub b: Point3<f32>,
    pub c: Point3<f32>,
}

impl Triangle {
    pub fn new<P>(a: P, b: P, c: P) -> Self
    where
        P: Into<Point3<f32>>,
    {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

impl approx::AbsDiffEq for Triangle {
    type Epsilon = <f32 as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Triangle, epsilon: Self::Epsilon) -> bool {
        [self.a, self.b, self.c]
            .abs_diff_eq(&[other.a, other.b, other.c], epsilon)
    }
}

impl approx::RelativeEq for Triangle {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Triangle,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        [self.a, self.b, self.c].relative_eq(
            &[other.a, other.b, other.c],
            epsilon,
            max_relative,
        )
    }
}
