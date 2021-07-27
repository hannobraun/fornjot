use nalgebra::{vector, Point, Unit, Vector};

use crate::geometry::{
    aabb::Aabb,
    traits::{BoundingVolume, Geometry, Sample},
};

pub struct Sweep<Sketch> {
    pub sketch: Sketch,
    pub distance: f32,
}

impl<Sketch> Sweep<Sketch> {
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.distance = height;
        self
    }
}

impl<Sketch> BoundingVolume<3> for Sweep<Sketch>
where
    Sketch: BoundingVolume<2>,
{
    fn aabb(&self) -> Aabb<3> {
        self.sketch
            .aabb()
            .extend(-self.distance / 2.0, self.distance / 2.0)
    }
}

impl<Sketch> Geometry<3> for Sweep<Sketch>
where
    Sketch: Geometry<2>,
{
    fn sample(&self, point: impl Into<Point<f32, 3>>) -> Sample<3> {
        let point = point.into();

        let sample_xy = self.sketch.sample(point.xy());

        let d_xy = sample_xy.distance;
        let d_z = point.z.abs() - self.distance / 2.0;

        let w = Vector::from([f32::max(d_xy, 0.0), f32::max(d_z, 0.0)]);

        let distance = f32::min(f32::max(d_xy, d_z), 0.0) + w.magnitude();

        let normal_xy = sample_xy.normal;
        let normal = if d_z < 0.0 {
            vector![normal_xy.x, normal_xy.y, 0.0]
        } else if d_xy < 0.0 {
            vector![0.0, 0.0, point.z]
        } else {
            vector![normal_xy.x, normal_xy.y, point.z.signum()]
        };
        let normal = Unit::new_normalize(normal);

        Sample {
            point,
            distance,
            normal,
        }
    }
}

// `LinearExtrude` is covered by a bunch of unit tests in `cylinder`.
