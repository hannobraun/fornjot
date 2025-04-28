use fj_math::{Point, Vector};

pub struct Circle {
    pub a: Vector<3>,
    pub b: Vector<3>,
}

impl Circle {
    pub fn vector_from_local_point(
        &self,
        point: impl Into<Point<1>>,
    ) -> Vector<3> {
        let angle = point.into().t;
        let (sin, cos) = angle.sin_cos();

        self.a * cos + self.b * sin - self.a
    }
}
