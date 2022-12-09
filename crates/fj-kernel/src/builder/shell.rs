use std::array;

use fj_interop::ext::{ArrayExt, SliceExt};
use fj_math::Scalar;
use iter_fixed::IntoIteratorFixed;

use crate::{
    algorithms::transform::TransformObject,
    builder::{FaceBuilder, HalfEdgeBuilder, SurfaceBuilder},
    insert::Insert,
    objects::{Face, FaceSet, HalfEdge, Objects, Shell, Vertex},
    partial::{
        Partial, PartialCurve, PartialCycle, PartialFace, PartialGlobalEdge,
        PartialHalfEdge, PartialObject, PartialSurface, PartialSurfaceVertex,
        PartialVertex,
    },
    services::Service,
    storage::Handle,
};

/// API for building a [`Shell`]
///
/// Also see [`Shell::builder`].
pub struct ShellBuilder {
    /// The faces that make up the [`Shell`]
    pub faces: FaceSet,
}

impl ShellBuilder {
    /// Build the [`Shell`] with the provided faces
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = Handle<Face>>,
    ) -> Self {
        self.faces.extend(faces);
        self
    }

    /// Create a cube from the length of its edges
    pub fn with_cube_from_edge_length(
        mut self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let edge_length = edge_length.into();

        // Let's define some short-hands. We're going to need them a lot.
        const Z: Scalar = Scalar::ZERO;
        let h = edge_length / 2.;

        let bottom = {
            let surface =
                objects.surfaces.xy_plane().translate([Z, Z, -h], objects);

            PartialFace::default()
                .with_exterior_polygon_from_points(
                    surface,
                    [[-h, -h], [h, -h], [h, h], [-h, h]],
                )
                .build(objects)
                .insert(objects)
        };

        let (sides, top_edges) = {
            let surfaces = bottom
                .exterior()
                .half_edges()
                .map(|half_edge| {
                    let [a, b] = half_edge
                        .vertices()
                        .clone()
                        .map(|vertex| vertex.global_form().position());
                    let c = a + [Z, Z, edge_length];

                    PartialSurface::plane_from_points([a, b, c])
                        .build(objects)
                        .insert(objects)
                })
                .collect::<Vec<_>>();

            let bottoms = bottom
                .exterior()
                .half_edges()
                .zip(&surfaces)
                .map(|(half_edge, surface)| {
                    let global_edge = Partial::from_full_entry_point(
                        half_edge.global_form().clone(),
                    )
                    .read()
                    .clone();

                    Partial::from_partial(
                        PartialHalfEdge {
                            vertices: global_edge.vertices.clone().map(
                                |global_vertex| {
                                    Partial::from_partial(PartialVertex {
                                        curve: Partial::from_partial(
                                            PartialCurve {
                                                global_form: global_edge
                                                    .curve
                                                    .clone(),
                                                ..Default::default()
                                            },
                                        ),
                                        surface_form: Partial::from_partial(
                                            PartialSurfaceVertex {
                                                global_form: global_vertex,
                                                ..Default::default()
                                            },
                                        ),
                                        ..Default::default()
                                    })
                                },
                            ),
                            global_form: Partial::from_partial(
                                PartialGlobalEdge {
                                    curve: global_edge.curve,
                                    vertices: global_edge.vertices,
                                },
                            ),
                        }
                        .update_as_line_segment_from_points(
                            Partial::from_full_entry_point(surface.clone()),
                            [[Z, Z], [edge_length, Z]],
                        ),
                    )
                })
                .collect::<Vec<_>>();

            let sides_up = bottoms
                .clone()
                .into_iter()
                .zip(&surfaces)
                .map(|(bottom, surface): (Partial<HalfEdge>, _)| {
                    let [_, from] = &bottom.read().vertices;

                    let from = from.read().surface_form.clone();
                    let to = PartialSurfaceVertex {
                        position: Some(
                            from.read().position.unwrap() + [Z, edge_length],
                        ),
                        surface: Partial::from_full_entry_point(
                            surface.clone(),
                        ),
                        ..Default::default()
                    };

                    let vertices = [
                        PartialVertex {
                            curve: Partial::from_partial(PartialCurve {
                                surface: from.read().surface.clone(),
                                ..Default::default()
                            }),
                            surface_form: from.clone(),
                            ..Default::default()
                        },
                        PartialVertex {
                            curve: Partial::from_partial(PartialCurve {
                                surface: to.surface.clone(),
                                ..Default::default()
                            }),
                            surface_form: Partial::from_partial(to),
                            ..Default::default()
                        },
                    ]
                    .map(Partial::<Vertex>::from_partial);

                    let global_curve = {
                        let [vertex, _] = &vertices;
                        vertex.read().curve.read().global_form.clone()
                    };
                    let global_vertices =
                        vertices.each_ref_ext().map(|vertex| {
                            vertex
                                .read()
                                .surface_form
                                .read()
                                .global_form
                                .clone()
                        });

                    Partial::from_partial(
                        PartialHalfEdge {
                            vertices,
                            global_form: Partial::from_partial(
                                PartialGlobalEdge {
                                    curve: global_curve,
                                    vertices: global_vertices,
                                },
                            ),
                        }
                        .update_as_line_segment(),
                    )
                })
                .collect::<Vec<_>>();

            let sides_down = {
                let mut sides_up_prev = sides_up.clone();
                sides_up_prev.rotate_right(1);

                bottoms
                    .clone()
                    .into_iter()
                    .zip(sides_up_prev)
                    .zip(&surfaces)
                    .map(
                        |((bottom, side_up_prev), surface): (
                            (_, Partial<HalfEdge>),
                            _,
                        )| {
                            let [_, from] =
                                side_up_prev.read().vertices.clone();
                            let [to, _] = bottom.read().vertices.clone();

                            let to = to.read().surface_form.clone();
                            let from = PartialSurfaceVertex {
                                position: Some(
                                    to.read().position.unwrap()
                                        + [Z, edge_length],
                                ),
                                surface: Partial::from_full_entry_point(
                                    surface.clone(),
                                ),
                                global_form: from
                                    .read()
                                    .surface_form
                                    .read()
                                    .global_form
                                    .clone(),
                            };

                            let curve = PartialCurve {
                                global_form: side_up_prev
                                    .read()
                                    .curve()
                                    .read()
                                    .global_form
                                    .clone(),
                                ..Default::default()
                            };

                            let vertices = [
                                PartialVertex {
                                    curve: Partial::from_partial(
                                        PartialCurve {
                                            surface: from.surface.clone(),
                                            ..curve.clone()
                                        },
                                    ),
                                    surface_form: Partial::from_partial(from),
                                    ..Default::default()
                                },
                                PartialVertex {
                                    curve: Partial::from_partial(
                                        PartialCurve {
                                            surface: to.read().surface.clone(),
                                            ..curve.clone()
                                        },
                                    ),
                                    surface_form: to.clone(),
                                    ..Default::default()
                                },
                            ]
                            .map(Partial::<Vertex>::from_partial);

                            let global_vertices =
                                vertices.each_ref_ext().map(|vertex| {
                                    vertex
                                        .read()
                                        .surface_form
                                        .read()
                                        .global_form
                                        .clone()
                                });

                            Partial::from_partial(
                                PartialHalfEdge {
                                    vertices,
                                    global_form: Partial::from_partial(
                                        PartialGlobalEdge {
                                            vertices: global_vertices,
                                            curve: curve.global_form,
                                        },
                                    ),
                                }
                                .update_as_line_segment(),
                            )
                        },
                    )
                    .collect::<Vec<_>>()
            };

            let tops = sides_up
                .clone()
                .into_iter()
                .zip(sides_down.clone())
                .map(|(side_up, side_down): (_, Partial<HalfEdge>)| {
                    let [_, from] = side_up.read().vertices.clone();
                    let [to, _] = side_down.read().vertices.clone();

                    let from = from.read().surface_form.clone();
                    let to = to.read().surface_form.clone();

                    let from = PartialVertex {
                        curve: Partial::from_partial(PartialCurve {
                            surface: from.read().surface.clone(),
                            ..Default::default()
                        }),
                        surface_form: from.clone(),
                        ..Default::default()
                    };
                    let to = PartialVertex {
                        curve: Partial::from_partial(PartialCurve {
                            surface: to.read().surface.clone(),
                            ..Default::default()
                        }),
                        surface_form: to.clone(),
                        ..Default::default()
                    };

                    let vertices =
                        [from, to].map(Partial::<Vertex>::from_partial);
                    let global_curve = {
                        let [vertex, _] = &vertices;
                        vertex.read().curve.read().global_form.clone()
                    };
                    let global_vertices =
                        vertices.each_ref_ext().map(|vertex| {
                            vertex
                                .read()
                                .surface_form
                                .read()
                                .global_form
                                .clone()
                        });

                    Partial::from_partial(
                        PartialHalfEdge {
                            vertices,
                            global_form: Partial::from_partial(
                                PartialGlobalEdge {
                                    curve: global_curve,
                                    vertices: global_vertices,
                                },
                            ),
                        }
                        .update_as_line_segment(),
                    )
                })
                .collect::<Vec<_>>();

            let sides = bottoms
                .into_iter()
                .zip(sides_up)
                .zip(tops.clone())
                .zip(sides_down)
                .map(|(((bottom, side_up), top), side_down)| {
                    let mut cycle = PartialCycle::default();
                    cycle.half_edges.extend([bottom, side_up, top, side_down]);

                    let face = PartialFace {
                        exterior: Partial::from_partial(cycle),
                        ..Default::default()
                    };
                    face.build(objects).insert(objects)
                })
                .collect::<Vec<_>>();

            (sides, tops)
        };

        let top = {
            let surface =
                objects.surfaces.xy_plane().translate([Z, Z, h], objects);

            let mut top_edges = top_edges;
            top_edges.reverse();

            let surface_vertices = {
                let points = [[-h, -h], [-h, h], [h, h], [h, -h]];

                let mut edges = top_edges.iter();
                let half_edges = array::from_fn(|_| edges.next().unwrap());

                let [a, b, c, d] = points
                    .into_iter_fixed()
                    .zip(half_edges)
                    .collect::<[_; 4]>()
                    .map(|(point, edge)| {
                        let [vertex, _] = edge.read().vertices.clone();
                        let global_vertex = vertex
                            .read()
                            .surface_form
                            .read()
                            .global_form
                            .clone();

                        PartialSurfaceVertex {
                            position: Some(point.into()),
                            surface: Partial::from_full_entry_point(
                                surface.clone(),
                            ),
                            global_form: global_vertex,
                        }
                        .build(objects)
                        .insert(objects)
                    });

                [a.clone(), b, c, d, a]
            };

            let mut edges = Vec::new();
            for (surface_vertices, edge) in surface_vertices
                .as_slice()
                .array_windows_ext()
                .zip(top_edges)
            {
                let global_edge = edge.read().global_form.clone();

                let vertices = edge
                    .read()
                    .vertices
                    .each_ref_ext()
                    .into_iter_fixed()
                    .zip(surface_vertices.clone())
                    .collect::<[_; 2]>()
                    .map(|(vertex, surface_form)| PartialVertex {
                        position: vertex.read().position,
                        curve: Partial::from_partial(PartialCurve {
                            surface: Partial::from_full_entry_point(
                                surface_form.surface().clone(),
                            ),
                            global_form: vertex
                                .read()
                                .curve
                                .read()
                                .global_form
                                .clone(),
                            ..Default::default()
                        }),
                        surface_form: Partial::from_full_entry_point(
                            surface_form,
                        ),
                    });

                edges.push(Partial::from_partial(
                    PartialHalfEdge {
                        vertices: vertices.map(Partial::from_partial),
                        global_form: Partial::from_partial(PartialGlobalEdge {
                            curve: global_edge.read().curve.clone(),
                            vertices: global_edge.read().vertices.clone(),
                        }),
                    }
                    .update_as_line_segment(),
                ));
            }

            let face = PartialFace {
                exterior: Partial::from_partial(PartialCycle::new(edges)),
                ..Default::default()
            };
            face.build(objects).insert(objects)
        };

        self.faces.extend([bottom]);
        self.faces.extend(sides);
        self.faces.extend([top]);

        self
    }

    /// Build the [`Shell`]
    pub fn build(self, objects: &mut Service<Objects>) -> Handle<Shell> {
        Shell::new(self.faces).insert(objects)
    }
}
