use std::array;

use fj_interop::ext::{ArrayExt, SliceExt};
use fj_math::Scalar;

use crate::{
    algorithms::transform::TransformObject,
    objects::{
        Curve, Cycle, Face, Faces, HalfEdge, Objects, Shell, Surface,
        SurfaceVertex, Vertex,
    },
    partial::HasPartial,
    storage::Handle,
};

/// API for building a [`Shell`]
///
/// Also see [`Shell::builder`].
pub struct ShellBuilder<'a> {
    /// The stores that the created objects are put in
    pub objects: &'a Objects,

    /// The faces that make up the [`Shell`]
    pub faces: Faces,
}

impl<'a> ShellBuilder<'a> {
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
    ) -> Self {
        let edge_length = edge_length.into();

        // Let's define some short-hands. We're going to need them a lot.
        const Z: Scalar = Scalar::ZERO;
        let h = edge_length / 2.;

        let bottom = {
            let surface = self
                .objects
                .surfaces
                .xy_plane()
                .translate([Z, Z, -h], self.objects)
                .unwrap();

            Face::builder(self.objects)
                .with_surface(surface)
                .with_exterior_polygon_from_points([
                    [-h, -h],
                    [h, -h],
                    [h, h],
                    [-h, h],
                ])
                .build()
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

                    self.objects
                        .surfaces
                        .insert(Surface::plane_from_points([a, b, c]))
                })
                .collect::<Vec<_>>();

            let bottoms = bottom
                .exterior()
                .half_edges()
                .zip(&surfaces)
                .map(|(half_edge, surface)| {
                    HalfEdge::partial()
                        .with_surface(Some(surface.clone()))
                        .with_global_form(Some(half_edge.global_form().clone()))
                        .as_line_segment_from_points([[Z, Z], [edge_length, Z]])
                        .build(self.objects)
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
                    let to = SurfaceVertex::partial()
                        .with_position(Some(from.position() + [Z, edge_length]))
                        .with_surface(Some(surface.clone()));

                    HalfEdge::partial()
                        .with_vertices(Some([
                            Vertex::partial().with_surface_form(Some(from)),
                            Vertex::partial().with_surface_form(Some(to)),
                        ]))
                        .as_line_segment()
                        .build(self.objects)
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
                        let from = SurfaceVertex::partial()
                            .with_position(Some(
                                to.position() + [Z, edge_length],
                            ))
                            .with_surface(Some(surface.clone()))
                            .with_global_form(Some(from.global_form().clone()));

                        let curve = Curve::partial().with_global_form(Some(
                            side_up_prev.curve().global_form().clone(),
                        ));

                        HalfEdge::partial()
                            .with_curve(Some(curve))
                            .with_vertices(Some([
                                Vertex::partial().with_surface_form(Some(from)),
                                Vertex::partial().with_surface_form(Some(to)),
                            ]))
                            .as_line_segment()
                            .build(self.objects)
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

                    let from = Vertex::partial().with_surface_form(Some(from));
                    let to = Vertex::partial().with_surface_form(Some(to));

                    HalfEdge::partial()
                        .with_vertices(Some([from, to]))
                        .as_line_segment()
                        .build(self.objects)
                        .unwrap()
                })
                .collect::<Vec<_>>();

            let sides = bottoms
                .into_iter()
                .zip(sides_up)
                .zip(tops.clone())
                .zip(sides_down)
                .zip(surfaces)
                .map(|((((bottom, side_up), top), side_down), surface)| {
                    let cycle = Cycle::partial()
                        .with_surface(Some(surface))
                        .with_half_edges([bottom, side_up, top, side_down])
                        .build(self.objects)
                        .unwrap();

                    Face::builder(self.objects).with_exterior(cycle).build()
                });

            (sides, tops)
        };

        let top = {
            let surface = self
                .objects
                .surfaces
                .xy_plane()
                .translate([Z, Z, h], self.objects)
                .unwrap();

            let mut top_edges = top_edges;
            top_edges.reverse();

            let surface_vertices = {
                let points = [[-h, -h], [-h, h], [h, h], [h, -h]];

                let mut edges = top_edges.iter();
                let half_edges = array::from_fn(|_| edges.next().unwrap());

                let [a, b, c, d] =
                    points.zip_ext(half_edges).map(|(point, edge)| {
                        let vertex = edge.back();

                        SurfaceVertex::partial()
                            .with_position(Some(point))
                            .with_surface(Some(surface.clone()))
                            .with_global_form(Some(
                                vertex.global_form().clone(),
                            ))
                            .build(self.objects)
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
                    .zip_ext(surface_vertices.clone())
                    .map(|(vertex, surface_form)| {
                        Vertex::partial()
                            .with_position(Some(vertex.position()))
                            .with_surface_form(Some(surface_form))
                    });

                edges.push(
                    HalfEdge::partial()
                        .with_vertices(Some(vertices))
                        .with_global_form(Some(edge.global_form().clone()))
                        .as_line_segment()
                        .build(self.objects)
                        .unwrap(),
                );
            }

            Face::builder(self.objects)
                .with_exterior(
                    self.objects.cycles.insert(Cycle::new(surface, edges)),
                )
                .build()
        };

        self.faces.extend([bottom]);
        self.faces.extend(sides);
        self.faces.extend([top]);

        self
    }

    /// Build the [`Shell`]
    pub fn build(self) -> Handle<Shell> {
        self.objects.shells.insert(Shell::new(self.faces))
    }
}
