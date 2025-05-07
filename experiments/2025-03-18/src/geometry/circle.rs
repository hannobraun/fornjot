use fj_math::{Point, Scalar, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub a: Vector<3>,
    pub b: Vector<3>,
}

impl Circle {
    pub fn radius(&self) -> Scalar {
        self.a.magnitude()
    }

    pub fn vector_from_local_point(
        &self,
        point: impl Into<Point<1>>,
    ) -> Vector<3> {
        let angle = point.into().t;
        let (sin, cos) = angle.sin_cos();

        self.a * cos + self.b * sin - self.a
    }

    pub fn project_vector(&self, vector: impl Into<Vector<3>>) -> Point<1> {
        let vector = self.a + vector.into();

        let [a, b] =
            [&self.a, &self.b].map(|v| vector.scalar_projection_onto(v));

        let atan = Scalar::atan2(b, a);
        let coord = if atan >= Scalar::ZERO {
            atan
        } else {
            atan + Scalar::TAU
        };

        Point::from([coord])
    }
}
