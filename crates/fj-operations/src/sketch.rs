use std::ops::Deref;

use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    builder::{CycleBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Objects, Sketch},
    partial::{
        Partial, PartialCycle, PartialFace, PartialHalfEdge, PartialObject,
        PartialSketch,
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
                let surface = Partial::from(surface);

                let half_edge = {
                    let mut half_edge = PartialHalfEdge::default();
                    half_edge.update_as_circle_from_radius(circle.radius());

                    Partial::from_partial(half_edge)
                };
                let exterior = {
                    let mut cycle = PartialCycle {
                        surface: surface.clone(),
                        ..Default::default()
                    };
                    cycle.half_edges.push(half_edge);
                    Partial::from_partial(cycle)
                };

                PartialFace {
                    surface,
                    exterior,
                    color: Some(Color(self.color())),
                    ..Default::default()
                }
            }
            fj::Chain::PolyChain(poly_chain) => {
                let segments = poly_chain.to_segments();
                assert!(
                    !segments.is_empty(),
                    "Attempted to compute a Brep from an empty sketch"
                );

                let exterior = {
                    let mut cycle = PartialCycle {
                        surface: Partial::from(surface.clone()),
                        ..Default::default()
                    };
                    let mut line_segments = vec![];
                    let mut arcs = vec![];
                    poly_chain.to_segments().into_iter().for_each(
                        |fj::SketchSegment { endpoint, route }| {
                            let endpoint = Point::from(endpoint);
                            match route {
                                fj::SketchSegmentRoute::Direct => {
                                    line_segments.push(
                                        cycle
                                            .add_half_edge_from_point_to_start(
                                                endpoint,
                                            ),
                                    );
                                }
                                fj::SketchSegmentRoute::Arc { angle } => {
                                    arcs.push((
                                        cycle
                                            .add_half_edge_from_point_to_start(
                                                endpoint,
                                            ),
                                        angle,
                                    ));
                                }
                            }
                        },
                    );
                    line_segments.into_iter().for_each(|mut half_edge| {
                        half_edge.write().update_as_line_segment()
                    });
                    arcs.into_iter().for_each(|(mut half_edge, angle)| {
                        half_edge.write().update_as_arc(angle.rad())
                    });
                    Partial::from_partial(cycle)
                };

                PartialFace {
                    surface: Partial::from(surface),
                    exterior,
                    color: Some(Color(self.color())),
                    ..Default::default()
                }
            }
        };

        let sketch = PartialSketch {
            faces: vec![Partial::from_partial(face)],
        }
        .build(objects)
        .insert(objects);
        sketch.deref().clone()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        match self.chain() {
            fj::Chain::Circle(circle) => Aabb {
                min: Point::from([-circle.radius(), -circle.radius(), 0.0]),
                max: Point::from([circle.radius(), circle.radius(), 0.0]),
            },
            fj::Chain::PolyChain(poly_chain) => {
                let segments = poly_chain.to_segments();
                assert!(
                    !segments.is_empty(),
                    "Attempted to compute a bounding box from an empty sketch"
                );

                let mut points = vec![];

                let mut start_point = segments[segments.len() - 1].endpoint;
                segments.iter().for_each(|segment| {
                    match segment.route {
                        fj::SketchSegmentRoute::Direct => (),
                        fj::SketchSegmentRoute::Arc { angle } => {
                            use std::f64::consts::PI;
                            let arc = fj_math::Arc::from_endpoints_and_angle(
                                start_point,
                                segment.endpoint,
                                fj_math::Scalar::from_f64(angle.rad()),
                            );
                            for circle_minmax_angle in
                                [0., PI / 2., PI, 3. * PI / 2.]
                            {
                                let mm_angle = fj_math::Scalar::from_f64(
                                    circle_minmax_angle,
                                );
                                if arc.start_angle < mm_angle
                                    && mm_angle < arc.end_angle
                                {
                                    points.push(
                                        arc.center
                                            + [
                                                arc.radius
                                                    * circle_minmax_angle.cos(),
                                                arc.radius
                                                    * circle_minmax_angle.sin(),
                                            ],
                                    );
                                }
                            }
                        }
                    }
                    points.push(Point::from(segment.endpoint));
                    start_point = segment.endpoint;
                });

                Aabb::<3>::from_points(points.into_iter().map(Point::to_xyz))
            }
        }
    }
}
