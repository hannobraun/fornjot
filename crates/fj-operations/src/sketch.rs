use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::{Cycle, Edge, Face, Surface},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Point, Scalar};

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Vec<Face>;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let surface = Surface::xy_plane();

        let sketch = match self.chain() {
            fj::Chain::Circle(circle) => {
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let edge =
                    Edge::circle_from_radius(Scalar::from_f64(circle.radius()));
                let cycle = Cycle { edges: vec![edge] };

                Face::new(surface, vec![cycle], Vec::new(), self.color())
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points =
                    poly_chain.to_points().into_iter().map(Point::from);

                Face::builder(surface)
                    .with_exterior_polygon(points)
                    .with_color(self.color())
                    .build()
            }
        };

        validate(vec![sketch], config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        match self.chain() {
            fj::Chain::Circle(circle) => Aabb {
                min: Point::from([-circle.radius(), -circle.radius(), 0.0]),
                max: Point::from([circle.radius(), circle.radius(), 0.0]),
            },
            fj::Chain::PolyChain(poly_chain) => Aabb::<3>::from_points(
                poly_chain
                    .to_points()
                    .into_iter()
                    .map(Point::from)
                    .map(Point::to_xyz),
            ),
        }
    }
}
