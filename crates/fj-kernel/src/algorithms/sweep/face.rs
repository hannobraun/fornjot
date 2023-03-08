use std::ops::Deref;

use fj_interop::ext::ArrayExt;
use fj_math::{Scalar, Vector};
use itertools::Itertools;

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    builder::{CycleBuilder, FaceBuilder},
    geometry::curve::GlobalPath,
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

        let mut top_face = PartialFace::new(objects);
        top_face.surface = Some(top_surface);
        top_face.color = Some(self.color());

        for (i, cycle) in bottom_face.all_cycles().cloned().enumerate() {
            let cycle = cycle.reverse(objects);

            let mut top_cycle = if i == 0 {
                top_face.exterior.clone()
            } else {
                top_face.add_interior(objects)
            };

            let mut original_edges = Vec::new();
            let mut top_edges = Vec::new();
            for (half_edge, next) in
                cycle.half_edges().cloned().circular_tuple_windows()
            {
                let (face, top_edge) = (
                    half_edge.clone(),
                    next.start_vertex(),
                    self.surface().deref(),
                    self.color(),
                )
                    .sweep_with_cache(path, cache, objects);

                faces.push(face);

                original_edges.push(half_edge);
                top_edges.push(Partial::from(top_edge));
            }

            top_cycle.write().connect_to_edges(top_edges, objects);

            for (bottom, top) in original_edges
                .into_iter()
                .zip(top_cycle.write().half_edges.iter_mut())
            {
                top.write().curve = Some(bottom.curve());

                let boundary = bottom.boundary();

                for (top, bottom) in
                    top.write().boundary.each_mut_ext().zip_ext(boundary)
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
