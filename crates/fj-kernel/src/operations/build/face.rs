use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    builder::HalfEdgeBuilder,
    objects::{Cycle, Face, GlobalEdge, Objects, Surface},
    operations::Insert,
    services::Service,
    storage::Handle,
};

use super::BuildSurface;

/// Build a [`Face`]
pub trait BuildFace {
    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        edges: [Option<Handle<GlobalEdge>>; 3],
        objects: &mut Service<Objects>,
    ) -> (Face, [Handle<GlobalEdge>; 3]) {
        let [a, b, c] = points.map(Into::into);

        let surface = Surface::plane_from_points([a, b, c]).insert(objects);
        let (exterior, global_edges) = {
            let half_edges = [[a, b], [b, c], [c, a]].zip_ext(edges).map(
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

            let global_edges =
                half_edges.map(|half_edge| half_edge.global_form().clone());

            (cycle, global_edges)
        };

        let face = Face::new(surface, exterior, [], None);

        (face, global_edges)
    }
}

impl BuildFace for Face {}
