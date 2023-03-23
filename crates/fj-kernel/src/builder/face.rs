use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::Point;

use crate::{
    objects::{Cycle, Face, GlobalEdge, Objects, Surface},
    operations::{BuildSurface, Insert},
    services::Service,
    storage::Handle,
};

use super::{CycleBuilder, HalfEdgeBuilder};

/// Builder API for [`Face`]
pub struct FaceBuilder {
    surface: Handle<Surface>,
    exterior: CycleBuilder,
    interiors: Vec<CycleBuilder>,
    color: Option<Color>,
}
impl FaceBuilder {
    /// Create an instance of `FaceBuilder`
    pub fn new(surface: Handle<Surface>) -> Self {
        Self {
            surface,
            exterior: CycleBuilder::new(),
            interiors: Vec::new(),
            color: None,
        }
    }

    /// Create a triangle
    pub fn triangle(
        points: [impl Into<Point<3>>; 3],
        edges: [Option<Handle<GlobalEdge>>; 3],
        objects: &mut Service<Objects>,
    ) -> (Handle<Face>, [Handle<GlobalEdge>; 3]) {
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

        let face = Face::new(surface, exterior, [], None).insert(objects);

        (face, global_edges)
    }

    /// Replace the face's exterior cycle
    pub fn with_exterior(mut self, exterior: CycleBuilder) -> Self {
        self.exterior = exterior;
        self
    }

    /// Add an interior cycle to the face
    pub fn with_interior(mut self, interior: CycleBuilder) -> Self {
        self.interiors.push(interior);
        self
    }

    /// Define the color of the face
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Build the face
    pub fn build(self, objects: &mut Service<Objects>) -> Face {
        let exterior = self.exterior.build(objects).insert(objects);
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.build(objects).insert(objects));

        Face::new(self.surface, exterior, interiors, self.color)
    }
}
