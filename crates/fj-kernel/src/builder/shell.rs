use fj_math::Point;

use crate::{
    insert::Insert,
    objects::{Cycle, Face, Objects, Shell},
    services::Service,
};

use super::{FaceBuilder, HalfEdgeBuilder, SurfaceBuilder};

/// Builder API for [`Shell`]
pub struct ShellBuilder {}

impl ShellBuilder {
    /// Create a tetrahedron from the provided points
    pub fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        objects: &mut Service<Objects>,
    ) -> Shell {
        let [a, b, c, d] = points.map(Into::into);

        let (bottom, [ab, bc, ca]) =
            FaceBuilder::triangle([a, b, c], [None, None, None], objects);
        let (front, [_, bd, da]) = {
            let surface =
                SurfaceBuilder::plane_from_points([a, b, d]).insert(objects);
            let (exterior, global_edges) = {
                let half_edges =
                    [([a, b], Some(ab)), ([b, d], None), ([d, a], None)].map(
                        |(points, global_form)| {
                            let mut builder =
                            HalfEdgeBuilder::line_segment_from_global_points(
                                points, &surface, None,
                            );

                            if let Some(global_form) = global_form {
                                builder = builder.with_global_form(global_form);
                            }

                            builder.build(objects).insert(objects)
                        },
                    );

                let cycle = Cycle::new(half_edges.clone()).insert(objects);

                let global_edges = half_edges
                    .map(|half_edges| half_edges.global_form().clone());

                (cycle, global_edges)
            };

            let face = Face::new(surface, exterior, [], None).insert(objects);

            (face, global_edges)
        };
        let (left, [_, _, dc]) = {
            let surface =
                SurfaceBuilder::plane_from_points([c, a, d]).insert(objects);
            let (exterior, global_edges) = {
                let half_edges =
                    [([c, a], Some(ca)), ([a, d], Some(da)), ([d, c], None)]
                        .map(|(points, global_form)| {
                            let mut builder =
                            HalfEdgeBuilder::line_segment_from_global_points(
                                points, &surface, None,
                            );

                            if let Some(global_form) = global_form {
                                builder = builder.with_global_form(global_form);
                            }

                            builder.build(objects).insert(objects)
                        });

                let cycle = Cycle::new(half_edges.clone()).insert(objects);

                let global_edges =
                    half_edges.map(|half_edge| half_edge.global_form().clone());

                (cycle, global_edges)
            };

            let face = Face::new(surface, exterior, [], None).insert(objects);

            (face, global_edges)
        };
        let back_right = {
            let surface =
                SurfaceBuilder::plane_from_points([b, c, d]).insert(objects);
            let exterior = {
                let half_edges = [([b, c], bc), ([c, d], dc), ([d, b], bd)]
                    .map(|(points, global_form)| {
                        HalfEdgeBuilder::line_segment_from_global_points(
                            points, &surface, None,
                        )
                        .with_global_form(global_form)
                        .build(objects)
                        .insert(objects)
                    });

                Cycle::new(half_edges).insert(objects)
            };

            Face::new(surface, exterior, [], None).insert(objects)
        };

        Shell::new([bottom, front, left, back_right])
    }
}
