use fj_math::{Line, Transform, Vector};

pub trait CurveGeometry {
    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry>;
}

impl CurveGeometry for () {
    fn translate(&self, _: Vector<3>) -> Box<dyn CurveGeometry> {
        Box::new(())
    }
}

impl CurveGeometry for Line<3> {
    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry> {
        let translated = self.transform(&Transform::translation(offset));
        Box::new(translated)
    }
}
