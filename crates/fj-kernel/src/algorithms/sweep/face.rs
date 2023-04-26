use std::ops::Deref;

use fj_math::{Scalar, Vector};
use itertools::Itertools;

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    builder::CycleBuilder,
    geometry::curve::GlobalPath,
    objects::{Face, Shell},
    operations::Insert,
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Face> {
    type Swept = Handle<Shell>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
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
                self.clone().reverse(services)
            }
        };
        faces.push(bottom_face.clone());

        let top_surface =
            bottom_face.surface().clone().translate(path, services);

        let mut exterior = None;
        let mut interiors = Vec::new();

        for (i, cycle) in bottom_face.all_cycles().cloned().enumerate() {
            let cycle = cycle.reverse(services);

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
                    .sweep_with_cache(path, cache, services);

                faces.push(face);

                top_edges.push((
                    top_edge,
                    half_edge.curve(),
                    half_edge.boundary(),
                ));
            }

            let top_cycle = CycleBuilder::connect_to_edges(top_edges, services)
                .build(&mut services.objects);

            if i == 0 {
                exterior = Some(top_cycle.insert(&mut services.objects));
            } else {
                interiors.push(top_cycle.insert(&mut services.objects));
            };
        }

        let top_face =
            Face::new(top_surface, exterior.unwrap(), interiors, self.color());

        let top_face = top_face.insert(&mut services.objects);
        faces.push(top_face);

        Shell::new(faces).insert(&mut services.objects)
    }
}
