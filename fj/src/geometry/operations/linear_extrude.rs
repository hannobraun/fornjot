use nalgebra::Point;

use crate::geometry::{
    aabb::Aabb,
    attributes::{BoundingVolume, Distance},
};

pub struct LinearExtrude<Sketch> {
    pub sketch: Sketch,
    pub height: f32,
}

impl<Sketch> LinearExtrude<Sketch> {
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<Sketch> BoundingVolume<3> for LinearExtrude<Sketch>
where
    Sketch: BoundingVolume<2>,
{
    fn aabb(&self) -> Aabb<3> {
        self.sketch
            .aabb()
            .extend(-self.height / 2.0, self.height / 2.0)
    }
}

impl<Sketch> Distance<3> for LinearExtrude<Sketch>
where
    Sketch: Distance<2>,
{
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32 {
        let point = point.into();

        let d_xy = self.sketch.distance(point.xy());
        let d_z = point.z.abs() - self.height / 2.0;

        if d_xy < 0.0 || d_z < 0.0 {
            f32::max(d_xy, d_z)
        } else {
            f32::min(d_xy, d_z)
        }
    }
}
