use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::{Cycle, Edge, Face, Surface},
    shape::{LocalForm, Shape},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Point, Scalar};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError> {
        let mut tmp = Shape::new();

        // Circles have just a single round edge with no vertices. So none need
        // to be added here.

        let edge = Edge::circle_from_radius(Scalar::from_f64(self.radius()));

        let cycle_local = Cycle {
            edges: vec![edge.clone()],
        };
        let cycle_canonical = Cycle::new(vec![edge.canonical().clone()]);

        let surface = Surface::xy_plane();
        let face = tmp
            .insert(Face::new(
                surface,
                vec![LocalForm::new(cycle_local, cycle_canonical)],
                Vec::new(),
                self.color(),
            ))
            .get();

        validate(vec![face], config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius(), -self.radius(), 0.0]),
            max: Point::from([self.radius(), self.radius(), 0.0]),
        }
    }
}
