use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::{Face, Surface},
    shape::Shape,
    validation::ValidationError,
};
use fj_math::{Aabb, Point};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(
        &self,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Shape, ValidationError> {
        let mut shape = Shape::new();

        let surface = Surface::xy_plane();
        let points = self.to_points().into_iter().map(Point::from);

        Face::builder(surface, &mut shape)
            .with_exterior_polygon(points)
            .with_color(self.color())
            .build()?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.to_points()
                .into_iter()
                .map(Point::from)
                .map(Point::to_xyz),
        )
    }
}
