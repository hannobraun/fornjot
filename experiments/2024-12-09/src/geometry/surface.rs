use crate::math::{Plane, Point, Vector};

/// # A trait for encoding surface geometry
///
/// So far, this is mostly cosmetic, as the only implementor is [`Plane`]. I've
/// started extracting the interface of that into this trait though, as a first
/// step towards eventually supporting other kinds of surfaces.
///
/// I'd expect that this trait would need to be expanded before that can be
/// fully realized.
pub trait SurfaceGeometry {
    /// # Convert a surface-local point to 3D
    fn point_from_local(&self, point: Point<2>) -> Point<3>;

    /// # Project a 3D point into the surface
    fn project_point(&self, point: Point<3>) -> Point<2>;

    /// # Flip the surface
    ///
    /// Maybe this can later merge with [`SurfaceGeometry::translate`] into a
    /// more general `transform` method.
    fn flip(&self) -> Box<dyn SurfaceGeometry>;

    /// # Translate the surface
    ///
    /// I expect this to transform into a more general `transform` method at
    /// some point. But so far, I haven't needed much more than this.
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
