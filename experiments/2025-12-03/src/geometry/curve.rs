use fj_math::Vector;

pub trait Curve {
    fn end(&self) -> Vector<3>;
    fn approx(&self) -> Vec<Vector<3>>;
}

pub struct Arc {
    pub end: Vector<3>,
    pub dir: Vector<3>,
}

impl Arc {
    pub fn to(end: impl Into<Vector<3>>, dir: impl Into<Vector<3>>) -> Self {
        Self {
            end: end.into(),
            dir: dir.into(),
        }
    }
}

impl Curve for Arc {
    fn end(&self) -> Vector<3> {
        self.end
    }

    fn approx(&self) -> Vec<Vector<3>> {
        // This is a placeholder for the actual approximation of the arc that
        // still needs to happen.
        let _ = self.dir;
        Vec::new()
    }
}
