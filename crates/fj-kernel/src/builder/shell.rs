use fj_math::Scalar;

use crate::{
    algorithms::transform::TransformObject,
    objects::{
        Curve, Cycle, Face, HalfEdge, Shell, Stores, Surface, SurfaceVertex,
        Vertex,
    },
    partial::HasPartial,
    storage::Handle,
};

/// API for building a [`Shell`]
///
/// Also see [`Shell::builder`].
pub struct ShellBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,
}

impl<'a> ShellBuilder<'a> {
    /// Create a cube from the length of its edges
    pub fn build_cube_from_edge_length(
        self,
        edge_length: impl Into<Scalar>,
    ) -> Shell {
        let edge_length = edge_length.into();

        // Let's define some short-hands. We're going to need them a lot.
        const Z: Scalar = Scalar::ZERO;
        let h = edge_length / 2.;

        let bottom = {
            let surface = self
                .stores
                .surfaces
                .insert(Surface::xy_plane())
                .translate([Z, Z, -h], self.stores);

            Face::builder(self.stores, surface)
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

                    self.stores
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
                        .build(self.stores)
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
                        .build(self.stores)
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

                        let curve = Handle::<Curve>::partial()
                            .with_global_form(Some(
                                side_up_prev.curve().global_form().clone(),
                            ));

                        HalfEdge::partial()
                            .with_curve(Some(curve))
                            .with_vertices(Some([
                                Vertex::partial().with_surface_form(Some(from)),
                                Vertex::partial().with_surface_form(Some(to)),
                            ]))
                            .as_line_segment()
                            .build(self.stores)
                    })
                    .collect::<Vec<_>>()
            };

            let tops = sides_up
                .clone()
                .into_iter()
                .zip(sides_down.clone())
                .zip(&surfaces)
                .map(|((side_up, side_down), surface)| {
                    let [_, from] = side_up.vertices();
                    let [to, _] = side_down.vertices();

                    let from = from.surface_form().clone();
                    let to = SurfaceVertex::partial()
                        .with_position(Some(
                            from.position() + [-edge_length, Z],
                        ))
                        .with_surface(Some(surface.clone()))
                        .with_global_form(Some(to.global_form().clone()));

                    let from = Vertex::partial().with_surface_form(Some(from));
                    let to = Vertex::partial().with_surface_form(Some(to));

                    HalfEdge::partial()
                        .with_vertices(Some([from, to]))
                        .as_line_segment()
                        .build(self.stores)
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
                        .build(self.stores);

                    Face::from_exterior(cycle)
                });

            (sides, tops)
        };

        let top = {
            let surface = self
                .stores
                .surfaces
                .insert(Surface::xy_plane())
                .translate([Z, Z, h], self.stores);

            let points = [[-h, -h], [-h, h], [h, h], [h, -h], [-h, -h]];

            let mut top_edges = top_edges;
            top_edges.reverse();

            let mut edges = Vec::new();
            for (points, edge) in points.windows(2).zip(top_edges) {
                // This can't panic, as we passed `2` to `windows`. Can be
                // cleaned up, once `array_windows` is stable.
                let points = [points[0], points[1]];

                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [point_a, point_b] = points;
                let [vertex_a, vertex_b] = edge.vertices().clone();
                let vertices = [(point_a, vertex_a), (point_b, vertex_b)].map(
                    |(point, vertex)| {
                        let surface_form = SurfaceVertex::partial()
                            .with_position(Some(point))
                            .with_surface(Some(surface.clone()))
                            .with_global_form(Some(
                                vertex.global_form().clone(),
                            ))
                            .build(self.stores);
                        Vertex::partial()
                            .with_position(Some(vertex.position()))
                            .with_surface_form(Some(surface_form))
                    },
                );

                edges.push(
                    HalfEdge::partial()
                        .with_vertices(Some(vertices))
                        .with_global_form(Some(edge.global_form().clone()))
                        .as_line_segment()
                        .build(self.stores),
                );
            }

            Face::from_exterior(Cycle::new(surface, edges))
        };

        let mut faces = Vec::new();
        faces.push(bottom);
        faces.extend(sides);
        faces.push(top);

        Shell::new().with_faces(faces)
    }
}
