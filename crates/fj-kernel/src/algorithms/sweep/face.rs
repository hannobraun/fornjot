use std::ops::Deref;

use fj_interop::ext::ArrayExt;
use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    builder::{CycleBuilder, FaceBuilder},
    geometry::path::GlobalPath,
    insert::Insert,
    objects::{Face, Objects, Shell},
    partial::{Partial, PartialFace, PartialObject, PartialShell},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Face> {
    type Swept = Handle<Shell>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let path = path.into();

        let mut faces = Vec::new();

        let is_negative_sweep = {
            let u = match self.surface().geometry().u {
                GlobalPath::Circle(_) => todo!(
                    "Sweeping from faces defined in round surfaces is not \
                    supported"
                ),
                GlobalPath::Line(line) => line.direction(),
            };
            let v = self.surface().geometry().v;

            let normal = u.cross(&v);

            normal.dot(&path) < Scalar::ZERO
        };

        let bottom_face = {
            if is_negative_sweep {
                self.clone()
            } else {
                self.clone().reverse(objects)
            }
        };
        faces.push(bottom_face.clone());

        let top_surface =
            bottom_face.surface().clone().translate(path, objects);
        let mut top_face = PartialFace {
            color: Some(self.color()),
            ..PartialFace::default()
        };

        for (i, cycle) in bottom_face.all_cycles().cloned().enumerate() {
            let cycle = cycle.reverse(objects);

            let mut top_cycle = if i == 0 {
                top_face.exterior.clone()
            } else {
                top_face.add_interior()
            };

            let mut original_edges = Vec::new();
            let mut top_edges = Vec::new();
            for half_edge in cycle.half_edges().cloned() {
                let (face, top_edge) =
                    (half_edge.clone(), self.surface().deref(), self.color())
                        .sweep_with_cache(path, cache, objects);

                faces.push(face);

                original_edges.push(half_edge);
                top_edges.push(Partial::from(top_edge));
            }

            top_cycle.write().surface = Partial::from(top_surface.clone());

            top_cycle.write().connect_to_closed_edges(top_edges);

            for half_edge in &mut top_cycle.write().half_edges {
                for (_, surface_vertex) in &mut half_edge.write().vertices {
                    let mut surface_vertex = surface_vertex.write();
                    let global_point =
                        surface_vertex.global_form.read().position;

                    if surface_vertex.position.is_none() {
                        if let Some(global_point) = global_point {
                            surface_vertex.position = Some(
                                top_surface
                                    .geometry()
                                    .project_global_point(global_point),
                            );
                        }
                    }
                }
            }

            for (bottom, top) in original_edges
                .into_iter()
                .zip(top_cycle.write().half_edges.iter_mut())
            {
                top.write().curve.write().path =
                    Some(bottom.curve().path().into());

                let boundary = bottom.boundary();

                for ((top, _), bottom) in
                    top.write().vertices.each_mut_ext().zip_ext(boundary)
                {
                    *top = Some(bottom);
                }
            }
        }

        let top_face = top_face.build(objects).insert(objects);
        faces.push(top_face);

        let faces = faces.into_iter().map(Partial::from).collect();
        PartialShell { faces }.build(objects).insert(objects)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use fj_interop::{ext::SliceExt, mesh::Color};

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        builder::{CycleBuilder, HalfEdgeBuilder, SketchBuilder},
        insert::Insert,
        partial::{
            Partial, PartialFace, PartialHalfEdge, PartialObject, PartialSketch,
        },
        services::Services,
    };

    use super::Sweep;

    const TRIANGLE: [[f64; 2]; 3] = [[0., 0.], [1., 0.], [0., 1.]];

    const UP: [f64; 3] = [0., 0., 1.];
    const DOWN: [f64; 3] = [0., 0., -1.];

    #[test]
    fn sweep_up() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let sketch = {
            let mut sketch = PartialSketch::default();

            let mut face = sketch.add_face();
            face.write().exterior.write().surface =
                Partial::from(surface.clone());
            face.write()
                .exterior
                .write()
                .update_as_polygon_from_points(TRIANGLE);

            sketch
        };
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(UP, &mut services.objects);

        let mut bottom = PartialFace::default();
        bottom.exterior.write().surface = Partial::from(surface.clone());
        bottom
            .exterior
            .write()
            .update_as_polygon_from_points(TRIANGLE);
        let bottom = bottom
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let mut top = PartialFace::default();
        top.exterior.write().surface =
            Partial::from(surface.clone().translate(UP, &mut services.objects));
        top.exterior.write().update_as_polygon_from_points(TRIANGLE);
        let top = top
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge.update_as_line_segment_from_points([a, b]);
                half_edge
                    .infer_vertex_positions_if_necessary(&surface.geometry());

                half_edge
                    .build(&mut services.objects)
                    .insert(&mut services.objects)
            };
            let (face, _) = (half_edge, surface.deref(), Color::default())
                .sweep(UP, &mut services.objects);
            face
        });

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
    }

    #[test]
    fn sweep_down() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let sketch = {
            let mut sketch = PartialSketch::default();

            let mut face = sketch.add_face();
            face.write().exterior.write().surface =
                Partial::from(surface.clone());
            face.write()
                .exterior
                .write()
                .update_as_polygon_from_points(TRIANGLE);

            sketch
        };
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(DOWN, &mut services.objects);

        let bottom = {
            let surface =
                surface.clone().translate(DOWN, &mut services.objects);

            let mut bottom = PartialFace::default();

            bottom.exterior.write().surface = Partial::from(surface);
            bottom
                .exterior
                .write()
                .update_as_polygon_from_points(TRIANGLE);

            bottom
                .build(&mut services.objects)
                .insert(&mut services.objects)
                .reverse(&mut services.objects)
        };
        let top = {
            let mut top = PartialFace::default();

            top.exterior.write().surface = Partial::from(surface.clone());
            top.exterior.write().update_as_polygon_from_points(TRIANGLE);

            top.build(&mut services.objects)
                .insert(&mut services.objects)
        };

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge.update_as_line_segment_from_points([a, b]);
                half_edge
                    .infer_vertex_positions_if_necessary(&surface.geometry());

                half_edge
                    .build(&mut services.objects)
                    .insert(&mut services.objects)
                    .reverse(&mut services.objects)
            };
            let (face, _) = (half_edge, surface.deref(), Color::default())
                .sweep(DOWN, &mut services.objects);
            face
        });

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
    }
}
