use crate::math::{Plane, Point, Vector};

pub trait SurfaceGeometry {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn project_point(&self, point: Point<3>) -> Point<2>;
    fn flip(&self) -> Box<dyn SurfaceGeometry>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry>;
}

impl SurfaceGeometry for Plane {
    fn point_from_local(&self, point: Point<2>) -> Point<3> {
        self.point_from_local(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<2> {
        self.project_point(point)
    }

    fn flip(&self) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).flip())
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).translate(offset))
    }
}
