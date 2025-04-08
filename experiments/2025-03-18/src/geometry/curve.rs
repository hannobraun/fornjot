use fj_math::{Line, Point, Transform, Vector};

pub trait CurveGeometry {
    fn point_from_local(&self, point: Point<1>) -> Point<3>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry>;
}

impl CurveGeometry for Line<3> {
    fn point_from_local(&self, point: Point<1>) -> Point<3> {
        self.point_from_line_coords(point)
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry> {
        let translated = self.transform(&Transform::translation(offset));
        Box::new(translated)
    }
}
