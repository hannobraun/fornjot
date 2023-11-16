use std::ops::Deref;

use fj_math::{Scalar, Vector};

use crate::{
    algorithms::transform::TransformObject,
    geometry::GlobalPath,
    objects::{Cycle, Face, Region, Shell},
    operations::{
        build::BuildCycle, insert::Insert, join::JoinCycle, reverse::Reverse,
    },
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
        // Please note that this function uses the words "bottom" and "top" in a
        // specific sense:
        //
        // - "Bottom" refers to the origin of the sweep. The bottom face is the
        //   original face, or a face in the same place.
        // - "Top" refers to the location of the face that was created by
        //   translating the bottom face along the path.
        // - "Side" refers to new faces created in between bottom and top.
        //
        // These words are specifically *not* meant in the sense of z-axis
        // locations, and depending on the direction of `path`, the two meanings
        // might actually be opposite.

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
                self.clone().reverse(services).insert(services)
            }
        };
        faces.push(bottom_face.clone());

        let top_surface =
            bottom_face.surface().clone().translate(path, services);

        let mut top_exterior = None;
        let mut top_interiors = Vec::new();

        for (i, bottom_cycle) in bottom_face.region().all_cycles().enumerate() {
            let bottom_cycle = bottom_cycle.reverse(services);

            let mut top_edges = Vec::new();
            for bottom_half_edge_pair in bottom_cycle.half_edges().pairs() {
                let (bottom_half_edge, bottom_half_edge_next) =
                    bottom_half_edge_pair;

                let (side_face, top_edge) = (
                    bottom_half_edge.deref(),
                    bottom_half_edge_next.start_vertex(),
                    bottom_face.surface().deref(),
                    bottom_face.region().color(),
                )
                    .sweep_with_cache(path, cache, services);

                faces.push(side_face);

                top_edges.push((
                    top_edge,
                    bottom_half_edge.path(),
                    bottom_half_edge.boundary(),
                ));
            }

            let top_cycle = Cycle::empty()
                .add_joined_edges(top_edges, services)
                .insert(services);

            if i == 0 {
                top_exterior = Some(top_cycle);
            } else {
                top_interiors.push(top_cycle);
            };
        }

        let top_region = Region::new(
            top_exterior.unwrap(),
            top_interiors,
            bottom_face.region().color(),
        )
        .insert(services);

        let top_face = Face::new(top_surface, top_region).insert(services);
        faces.push(top_face);

        Shell::new(faces).insert(services)
    }
}
