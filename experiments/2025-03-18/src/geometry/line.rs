use fj_math::{Point, Vector};

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
