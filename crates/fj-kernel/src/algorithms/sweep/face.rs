use std::ops::Deref;

use fj_math::{Scalar, Vector};
use itertools::Itertools;

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    builder::CycleBuilder,
    geometry::curve::GlobalPath,
    insert::Insert,
    objects::{Cycle, Face, Objects, Shell},
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

        let mut exterior = None;
        let mut interiors = Vec::new();

        for (i, cycle) in bottom_face.all_cycles().cloned().enumerate() {
            let cycle = cycle.reverse(objects);

            let mut top_edges = Vec::new();
            for (half_edge, next) in
                cycle.half_edges().cloned().circular_tuple_windows()
            {
                let (face, top_edge) = (
                    half_edge.deref(),
                    next.start_vertex(),
                    self.surface().deref(),
                    self.color(),
                )
                    .sweep_with_cache(path, cache, objects);

                faces.push(face);

                top_edges.push((
                    top_edge,
                    half_edge.curve(),
                    half_edge.boundary(),
                ));
            }

            let (top_cycle, _) =
                Cycle::new([]).connect_to_edges(top_edges, objects);

            if i == 0 {
                exterior = Some(top_cycle.insert(objects));
            } else {
                interiors.push(top_cycle.insert(objects));
            };
        }

        top_face.exterior = exterior.unwrap();
        top_face.interiors = interiors;

        let top_face = top_face.build(objects).insert(objects);
        faces.push(top_face);

        let faces = faces.into_iter().map(Partial::from).collect();
        PartialShell { faces }.build(objects).insert(objects)
    }
}
