use fj_math::Vector;

pub trait CurveGeometry {
    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry>;
}

impl CurveGeometry for () {
    fn translate(&self, _: Vector<3>) -> Box<dyn CurveGeometry> {
        Box::new(())
    }
}
