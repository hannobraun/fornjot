use std::array;

use fj_interop::ext::{ArrayExt, SliceExt};
use fj_math::Scalar;
use iter_fixed::IntoIteratorFixed;

use crate::{
    algorithms::transform::TransformObject,
    builder::{FaceBuilder, HalfEdgeBuilder, SurfaceBuilder},
    insert::Insert,
    objects::{Cycle, Face, FaceSet, Objects, Shell},
    partial::{
        HasPartial, PartialCurve, PartialHalfEdge, PartialSurface,
        PartialSurfaceVertex, PartialVertex,
    },
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
        objects: &mut Objects,
    ) -> Self {
        let edge_length = edge_length.into();

        // Let's define some short-hands. We're going to need them a lot.
        const Z: Scalar = Scalar::ZERO;
        let h = edge_length / 2.;

        let bottom = {
            let surface = objects
                .surfaces
                .xy_plane()
                .translate([Z, Z, -h], objects)
                .unwrap();

            Face::partial()
                .with_surface(surface)
                .with_exterior_polygon_from_points([
                    [-h, -h],
                    [h, -h],
                    [h, h],
                    [-h, h],
                ])
                .build(objects)
                .unwrap()
                .insert(objects)
                .unwrap()
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
                        .unwrap()
                        .insert(objects)
                        .unwrap()
                })
                .collect::<Vec<_>>();

            let bottoms = bottom
                .exterior()
                .half_edges()
                .zip(&surfaces)
                .map(|(half_edge, surface)| {
                    PartialHalfEdge {
                        global_form: half_edge.global_form().clone().into(),
                        ..Default::default()
                    }
                    .update_as_line_segment_from_points(
                        surface.clone(),
                        [[Z, Z], [edge_length, Z]],
                    )
                    .build(objects)
                    .unwrap()
                    .insert(objects)
                    .unwrap()
                })
                .collect::<Vec<_>>();

            let sides_up = bottoms
                .clone()
                .into_iter()
                .zip(&surfaces)
                .map(|(bottom, surface)| {
                    let [_, from] = bottom.vertices();

                    let from = from.surface_form().clone();
                    let to = PartialSurfaceVertex {
                        position: Some(from.position() + [Z, edge_length]),
                        surface: Some(surface.clone()),
                        ..Default::default()
                    };

                    PartialHalfEdge {
                        vertices: [
                            PartialVertex {
                                surface_form: from.into(),
                                ..Default::default()
                            },
                            PartialVertex {
                                surface_form: to.into(),
                                ..Default::default()
                            },
                        ]
                        .map(Into::into),
                        ..Default::default()
                    }
                    .update_as_line_segment()
                    .build(objects)
                    .unwrap()
                    .insert(objects)
                    .unwrap()
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
                    .map(|((bottom, side_up_prev), surface)| {
                        let [_, from] = side_up_prev.vertices();
                        let [to, _] = bottom.vertices();

                        let to = to.surface_form().clone();
                        let from = PartialSurfaceVertex {
                            position: Some(to.position() + [Z, edge_length]),
                            surface: Some(surface.clone()),
                            global_form: from.global_form().clone().into(),
                        };

                        let curve = PartialCurve {
                            global_form: side_up_prev
                                .curve()
                                .global_form()
                                .clone()
                                .into(),
                            ..Default::default()
                        };

                        PartialHalfEdge {
                            curve: curve.into(),
                            vertices: [
                                PartialVertex {
                                    surface_form: from.into(),
                                    ..Default::default()
                                },
                                PartialVertex {
                                    surface_form: to.into(),
                                    ..Default::default()
                                },
                            ]
                            .map(Into::into),
                            ..Default::default()
                        }
                        .update_as_line_segment()
                        .build(objects)
                        .unwrap()
                        .insert(objects)
                        .unwrap()
                    })
                    .collect::<Vec<_>>()
            };

            let tops = sides_up
                .clone()
                .into_iter()
                .zip(sides_down.clone())
                .map(|(side_up, side_down)| {
                    let [_, from] = side_up.vertices();
                    let [to, _] = side_down.vertices();

                    let from = from.surface_form().clone();
                    let to = to.surface_form().clone();

                    let from = PartialVertex {
                        surface_form: from.into(),
                        ..Default::default()
                    };
                    let to = PartialVertex {
                        surface_form: to.into(),
                        ..Default::default()
                    };

                    PartialHalfEdge {
                        vertices: [from, to].map(Into::into),
                        ..Default::default()
                    }
                    .update_as_line_segment()
                    .build(objects)
                    .unwrap()
                    .insert(objects)
                    .unwrap()
                })
                .collect::<Vec<_>>();

            let sides = bottoms
                .into_iter()
                .zip(sides_up)
                .zip(tops.clone())
                .zip(sides_down)
                .map(|(((bottom, side_up), top), side_down)| {
                    let cycle = Cycle::partial()
                        .with_half_edges([bottom, side_up, top, side_down])
                        .build(objects)
                        .unwrap()
                        .insert(objects)
                        .unwrap();

                    Face::partial()
                        .with_exterior(cycle)
                        .build(objects)
                        .unwrap()
                        .insert(objects)
                        .unwrap()
                })
                .collect::<Vec<_>>();

            (sides, tops)
        };

        let top = {
            let surface = objects
                .surfaces
                .xy_plane()
                .translate([Z, Z, h], objects)
                .unwrap();

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
                        let vertex = edge.back();

                        PartialSurfaceVertex {
                            position: Some(point.into()),
                            surface: Some(surface.clone()),
                            global_form: vertex.global_form().clone().into(),
                        }
                        .build(objects)
                        .unwrap()
                        .insert(objects)
                        .unwrap()
                    });

                [a.clone(), b, c, d, a]
            };

            let mut edges = Vec::new();
            for (surface_vertices, edge) in surface_vertices
                .as_slice()
                .array_windows_ext()
                .zip(top_edges)
            {
                let vertices = edge
                    .vertices()
                    .each_ref_ext()
                    .into_iter_fixed()
                    .zip(surface_vertices.clone())
                    .collect::<[_; 2]>()
                    .map(|(vertex, surface_form)| PartialVertex {
                        position: Some(vertex.position()),
                        surface_form: surface_form.into(),
                        ..Default::default()
                    });

                edges.push(
                    PartialHalfEdge {
                        vertices: vertices.map(Into::into),
                        global_form: edge.global_form().clone().into(),
                        ..Default::default()
                    }
                    .update_as_line_segment()
                    .build(objects)
                    .unwrap()
                    .insert(objects)
                    .unwrap(),
                );
            }

            Face::partial()
                .with_exterior(Cycle::new(edges).insert(objects).unwrap())
                .build(objects)
                .unwrap()
                .insert(objects)
                .unwrap()
        };

        self.faces.extend([bottom]);
        self.faces.extend(sides);
        self.faces.extend([top]);

        self
    }

    /// Build the [`Shell`]
    pub fn build(self, objects: &mut Objects) -> Handle<Shell> {
        Shell::new(self.faces).insert(objects).unwrap()
    }
}
