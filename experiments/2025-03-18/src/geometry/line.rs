use fj_math::{Point, Vector};

#[derive(Clone, Copy)]
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
}
