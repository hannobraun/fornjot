use std::ops::Deref;

use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    builder::{FaceBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Cycle, Face, HalfEdge, Objects, Sketch},
    partial::{HasPartial, Replace},
    validate::ValidationError,
};
use fj_math::{Aabb, Point};

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        objects: &Objects,
        _: &mut DebugInfo,
    ) -> Result<Self::Brep, ValidationError> {
        let surface = objects.surfaces.xy_plane();

        let face = match self.chain() {
            fj::Chain::Circle(circle) => {
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let half_edge = {
                    let mut half_edge = HalfEdge::partial();
                    half_edge.replace(surface);
                    half_edge
                        .update_as_circle_from_radius(circle.radius(), objects)?
                        .build(objects)?
                        .insert(objects)?
                };
                let cycle = Cycle::new([half_edge]).insert(objects)?;

                Face::partial()
                    .with_exterior(cycle)
                    .with_color(Color(self.color()))
                    .build(objects)?
                    .insert(objects)?
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points = poly_chain
                    .to_segments()
                    .into_iter()
                    .map(|fj::SketchSegment::LineTo { point }| point)
                    .map(Point::from);

                Face::partial()
                    .with_surface(surface)
                    .with_exterior_polygon_from_points(points)
                    .with_color(Color(self.color()))
                    .build(objects)?
                    .insert(objects)?
            }
        };

        let sketch = Sketch::builder().with_faces([face]).build(objects);
        Ok(sketch.deref().clone())
    }

    fn bounding_volume(&self) -> Aabb<3> {
        match self.chain() {
            fj::Chain::Circle(circle) => Aabb {
                min: Point::from([-circle.radius(), -circle.radius(), 0.0]),
                max: Point::from([circle.radius(), circle.radius(), 0.0]),
            },
            fj::Chain::PolyChain(poly_chain) => Aabb::<3>::from_points(
                poly_chain
                    .to_segments()
                    .into_iter()
                    .map(|fj::SketchSegment::LineTo { point }| point)
                    .map(Point::from)
                    .map(Point::to_xyz),
            ),
        }
    }
}
