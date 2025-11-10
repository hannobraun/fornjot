use std::rc::Rc;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar, Vector};

use crate::geometry::{curve::CurveGeometry, increment::Increment};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub direction: Vector<3>,
}

impl Line {
    pub fn vector_from_local_point(
        &self,
        point: impl Into<Point<1>>,
    ) -> Vector<3> {
        self.direction * point.into().t
    }

    pub fn project_vector(&self, vector: impl Into<Vector<3>>) -> Point<1> {
        let t = vector.into().scalar_projection_onto(&self.direction)
            / self.direction.magnitude();
        Point::from([t])
    }
}

impl CurveGeometry for Line {
    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.vector_from_local_point(point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        self.project_vector(vector)
    }

    fn flip(&self) -> Rc<dyn CurveGeometry> {
        Rc::new(Line {
            direction: -self.direction,
        })
    }

    fn increment_at(
        &self,
        _: Point<1>,
        _: Tolerance,
        size_hint: Scalar,
    ) -> Increment<1> {
        Increment {
            inner: Vector::from([size_hint]),
        }
    }
}
