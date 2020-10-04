use crate::geometry::Triangle;

#[derive(Debug, PartialEq)]
pub struct Triangles(pub Vec<Triangle>);

impl approx::AbsDiffEq for Triangles {
    type Epsilon = <Triangle as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Triangle::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Triangles, epsilon: Self::Epsilon) -> bool {
        self.0.as_slice().abs_diff_eq(other.0.as_slice(), epsilon)
    }
}

impl approx::RelativeEq for Triangles {
    fn default_max_relative() -> Self::Epsilon {
        Triangle::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Triangles,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0
            .as_slice()
            .relative_eq(other.0.as_slice(), epsilon, max_relative)
    }
}
