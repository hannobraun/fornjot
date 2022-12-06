use std::{array, ops::Deref};

use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    builder::{FaceBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Cycle, Face, Objects, Sketch},
    partial::{HasPartial, MaybePartial, PartialHalfEdge, PartialVertex},
    partial2::{Partial, PartialCurve},
    services::Service,
};
use fj_math::{Aabb, Point};

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        objects: &mut Service<Objects>,
        _: &mut DebugInfo,
    ) -> Self::Brep {
        let surface = objects.surfaces.xy_plane();

        let face = match self.chain() {
            fj::Chain::Circle(circle) => {
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let half_edge = {
                    let half_edge = PartialHalfEdge {
                        vertices: array::from_fn(|_| {
                            MaybePartial::from(PartialVertex {
                                curve: Partial::from_partial(PartialCurve {
                                    surface: Partial::from_full_entry_point(
                                        surface.clone(),
                                    ),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            })
                        }),
                        ..Default::default()
                    };
                    half_edge
                        .update_as_circle_from_radius(circle.radius(), objects)
                        .build(objects)
                        .insert(objects)
                };
                let cycle = Cycle::new([half_edge]).insert(objects);

                Face::partial()
                    .with_exterior(cycle)
                    .with_color(Color(self.color()))
                    .build(objects)
                    .insert(objects)
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points = poly_chain
                    .to_segments()
                    .into_iter()
                    .map(|fj::SketchSegment::LineTo { point }| point)
                    .map(Point::from);

                Face::partial()
                    .with_exterior_polygon_from_points(surface, points)
                    .with_color(Color(self.color()))
                    .build(objects)
                    .insert(objects)
            }
        };

        let sketch = Sketch::builder().with_faces([face]).build(objects);
        sketch.deref().clone()
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
