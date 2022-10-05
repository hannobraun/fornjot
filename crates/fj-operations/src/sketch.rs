use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    algorithms::validate::{
        Validate, Validated, ValidationConfig, ValidationError,
    },
    objects::{Cycle, Face, HalfEdge, Sketch},
    partial::HasPartial,
    stores::Stores,
};
use fj_math::{Aabb, Point};

use crate::planes::Planes;

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        stores: &Stores,
        planes: &Planes,
        _: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let surface = planes.xy();

        let face = match self.chain() {
            fj::Chain::Circle(circle) => {
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let half_edge = HalfEdge::partial()
                    .as_circle_from_radius(surface.clone(), circle.radius())
                    .build(stores);
                let cycle = Cycle::new(surface, [half_edge]);

                Face::from_exterior(cycle).with_color(Color(self.color()))
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points =
                    poly_chain.to_points().into_iter().map(Point::from);

                Face::builder(stores, surface)
                    .with_exterior_polygon_from_points(points)
                    .build()
                    .with_color(Color(self.color()))
            }
        };

        let sketch = Sketch::new().with_faces([face]);
        sketch.validate_with_config(config)
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
