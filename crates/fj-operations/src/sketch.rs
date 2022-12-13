use std::ops::Deref;

use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    builder::{FaceBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Objects, Sketch},
    partial::{
        Partial, PartialCycle, PartialFace, PartialHalfEdge, PartialObject,
    },
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
                let half_edge = {
                    let surface = Partial::from_full_entry_point(surface);

                    let mut half_edge = PartialHalfEdge::default();

                    half_edge.curve().write().surface = surface.clone();

                    for vertex in &mut half_edge.vertices {
                        vertex.write().surface_form.write().surface =
                            surface.clone();
                    }

                    half_edge.update_as_circle_from_radius(circle.radius());

                    Partial::from_partial(half_edge)
                };
                let cycle =
                    Partial::from_partial(PartialCycle::new(vec![half_edge]));

                let face = PartialFace {
                    exterior: cycle,
                    color: Some(Color(self.color())),
                    ..Default::default()
                };
                face.build(objects).insert(objects)
            }
            fj::Chain::PolyChain(poly_chain) => {
                let points = poly_chain
                    .to_segments()
                    .into_iter()
                    .map(|fj::SketchSegment::LineTo { point }| point)
                    .map(Point::from);

                let mut face = PartialFace::default()
                    .with_exterior_polygon_from_points(surface, points);
                face.color = Some(Color(self.color()));

                face.build(objects).insert(objects)
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
