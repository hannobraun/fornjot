use fj_math::Vector;

pub trait Curve {
    fn end(&self) -> Vector<3>;
}

pub struct LineSegment {
    pub end: Vector<3>,
}

impl LineSegment {
    pub fn to(end: impl Into<Vector<3>>) -> Self {
        Self { end: end.into() }
    }
}

impl Curve for LineSegment {
    fn end(&self) -> Vector<3> {
        self.end
    }
}
