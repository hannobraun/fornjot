use std::{array, ops::Deref};

use fj_interop::{debug::DebugInfo, ext::ArrayExt, mesh::Color};
use fj_kernel::{
    builder::{FaceBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Objects, Sketch, Vertex},
    partial::{
        Partial, PartialCurve, PartialCycle, PartialFace, PartialGlobalEdge,
        PartialHalfEdge, PartialObject, PartialSurfaceVertex, PartialVertex,
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
                // Circles have just a single round edge with no vertices. So
                // none need to be added here.

                let half_edge = {
                    let surface = Partial::from_full_entry_point(surface);
                    let curve = Partial::from_partial(PartialCurve {
                        surface: surface.clone(),
                        ..Default::default()
                    });
                    let vertices = array::from_fn(|_| {
                        Partial::from_partial(PartialVertex {
                            curve: curve.clone(),
                            surface_form: Partial::from_partial(
                                PartialSurfaceVertex {
                                    surface: surface.clone(),
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        })
                    });
                    let global_vertices = vertices.each_ref_ext().map(
                        |vertex: &Partial<Vertex>| {
                            vertex
                                .read()
                                .surface_form
                                .read()
                                .global_form
                                .clone()
                        },
                    );

                    let half_edge = PartialHalfEdge {
                        vertices,
                        global_form: Partial::from_partial(PartialGlobalEdge {
                            curve: curve.read().global_form.clone(),
                            vertices: global_vertices,
                        }),
                    };
                    Partial::from_partial(
                        half_edge.update_as_circle_from_radius(circle.radius()),
                    )
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
