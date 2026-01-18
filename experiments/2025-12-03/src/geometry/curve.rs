use fj_math::Vector;

pub struct LineSegment {
    pub end: Vector<3>,
}

impl LineSegment {
    pub fn to(end: impl Into<Vector<3>>) -> Self {
        Self { end: end.into() }
    }
}
