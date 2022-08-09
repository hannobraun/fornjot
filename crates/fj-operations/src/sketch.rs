use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    algorithms::approx::Tolerance,
    objects::{Cycle, Edge, Face, Sketch, Surface},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Point, Scalar};

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let surface = Surface::xy_plane();

        let face = match self.chain() {
            fj::Chain::Circle(circle) => {
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let edge = Edge::build(surface)
                    .circle_from_radius(Scalar::from_f64(circle.radius()));
                let cycle = Cycle::new(surface).with_edges([edge]);

                Face::new(surface)
                    .with_exteriors([cycle])
                    .with_color(Color(self.color()))
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points =
                    poly_chain.to_points().into_iter().map(Point::from);

                Face::build(surface)
                    .polygon_from_points(points)
                    .into_face()
                    .with_color(Color(self.color()))
            }
        };

        let sketch = Sketch::new().with_faces([face]);
        validate(sketch, config)
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
