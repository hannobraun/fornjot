use std::{array, collections::VecDeque};

use fj_interop::ext::ArrayExt;

use crate::{
    geometry::path::SurfacePath,
    objects::{Cycle, Surface},
    partial::{MaybeSurfacePath, Partial, PartialCycle, PartialFace},
};

use super::SurfaceBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;

    /// Update the face's surface as a plane
    ///
    /// The plane geometry is inferred from three of the face's vertices. Also
    /// infers any undefined `SurfaceVertex` positions.
    ///
    /// # Panics
    ///
    /// Assumes that the face exterior has exactly three vertices to use. Panics
    /// otherwise. This is a temporary limitation, not a fundamental one. It
    /// could be overcome with some more work.
    fn update_surface_as_plane(&mut self) -> Partial<Surface>;

    /// Infer any undefined curves in the face
    fn infer_curves(&mut self);
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = Partial::from_partial(PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        });
        self.interiors.push(cycle.clone());
        cycle
    }

    fn update_surface_as_plane(&mut self) -> Partial<Surface> {
        let exterior = self.exterior.read();
        let mut vertices = exterior
            .half_edges
            .iter()
            .map(|half_edge| {
                let [(_, surface_vertex), _] = &half_edge.read().vertices;
                let global_position = surface_vertex
                    .read()
                    .global_form
                    .read()
                    .position
                    .expect("Need global position to infer plane");

                (surface_vertex.clone(), global_position)
            })
            .collect::<VecDeque<_>>();

        let (significant_vertices, surface) = {
            let first_three_vertices = array::from_fn(|_| {
                vertices
                    .pop_front()
                    .expect("Expected at least three vertices")
            });

            let first_three_points_global =
                first_three_vertices.each_ref_ext().map(|(_, point)| *point);

            let (first_three_points_surface, surface) = self
                .surface
                .write()
                .update_as_plane_from_points(first_three_points_global);

            let first_three_vertices = first_three_vertices
                .zip_ext(first_three_points_surface)
                .map(|((vertex, _), point_global)| (vertex, point_global));

            (first_three_vertices, surface)
        };
        let rest_of_vertices =
            vertices.into_iter().map(|(vertex, point_global)| {
                let point_surface = surface.project_global_point(point_global);
                (vertex, point_surface)
            });

        for (mut surface_vertex, point) in
            significant_vertices.into_iter().chain(rest_of_vertices)
        {
            surface_vertex.write().position = Some(point);
        }

        self.surface.clone()
    }

    fn infer_curves(&mut self) {
        for half_edge in &mut self.exterior.write().half_edges {
            let mut half_edge = half_edge.write();

            let mut curve = half_edge.curve.clone();
            let mut curve = curve.write();

            if let Some(path) = &mut curve.path {
                match path {
                    MaybeSurfacePath::Defined(_) => {
                        // Path is already defined. Nothing to infer.
                    }
                    MaybeSurfacePath::UndefinedCircle { .. } => todo!(
                        "Inferring undefined circles is not supported yet"
                    ),
                    MaybeSurfacePath::UndefinedLine => {
                        let points_surface =
                            half_edge.vertices.each_ref_ext().map(|vertex| {
                                vertex.1.read().position.expect(
                                    "Can't infer curve without surface points",
                                )
                            });
                        let (line, points_curve) =
                            SurfacePath::line_from_points(points_surface);

                        *path = MaybeSurfacePath::Defined(line);
                        for (vertex, point) in half_edge
                            .vertices
                            .each_mut_ext()
                            .zip_ext(points_curve)
                        {
                            vertex.0 = Some(point);
                        }
                    }
                }
            }
        }
    }
}
