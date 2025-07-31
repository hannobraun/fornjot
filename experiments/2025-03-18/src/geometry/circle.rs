use fj_interop::{CircleApproxParams, Tolerance};
use fj_math::{Point, Scalar, Vector};

use crate::geometry::curve::{CurveApprox, CurveGeometry};

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

impl CurveGeometry for Circle {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.vector_from_local_point(point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        self.project_vector(vector)
    }

    fn flip(&self) -> Box<dyn CurveGeometry> {
        Box::new(Circle {
            a: self.a,
            b: -self.b,
        })
    }

    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> CurveApprox {
        let curvature = CircleApproxParams::new(self.radius(), tolerance)
            .approx_circle(boundary)
            .collect();

        CurveApprox { curvature }
    }
}
