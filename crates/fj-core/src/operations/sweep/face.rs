use fj_interop::mesh::Color;
use fj_math::{Scalar, Vector};

use crate::{
    algorithms::transform::TransformObject,
    geometry::GlobalPath,
    objects::{Cycle, Face, Region, Shell, Surface},
    operations::{insert::Insert, reverse::Reverse},
    services::Services,
    storage::Handle,
};

use super::{SweepCache, SweepCycle};

/// # Sweep a [`Face`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepFace {
    /// # Sweep the [`Face`]
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Shell;
}

impl SweepFace for Face {
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Shell {
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

        let bottom_face = bottom_face(self, path, services).insert(services);
        faces.push(bottom_face.clone());

        let top_surface =
            bottom_face.surface().clone().translate(path, services);

        let top_exterior = sweep_cycle(
            bottom_face.region().exterior(),
            bottom_face.surface(),
            bottom_face.region().color(),
            &mut faces,
            path,
            cache,
            services,
        );

        let mut top_interiors = Vec::new();

        for bottom_cycle in bottom_face.region().interiors() {
            let top_cycle = sweep_cycle(
                bottom_cycle,
                bottom_face.surface(),
                bottom_face.region().color(),
                &mut faces,
                path,
                cache,
                services,
            );

            top_interiors.push(top_cycle);
        }

        let top_region = Region::new(
            top_exterior,
            top_interiors,
            bottom_face.region().color(),
        )
        .insert(services);

        let top_face = Face::new(top_surface, top_region).insert(services);
        faces.push(top_face);

        Shell::new(faces)
    }
}

fn bottom_face(face: &Face, path: Vector<3>, services: &mut Services) -> Face {
    let is_negative_sweep = {
        let u = match face.surface().geometry().u {
            GlobalPath::Circle(_) => todo!(
                "Sweeping from faces defined in rounded surfaces is not \
                    supported"
            ),
            GlobalPath::Line(line) => line.direction(),
        };
        let v = face.surface().geometry().v;

        let normal = u.cross(&v);

        normal.dot(&path) < Scalar::ZERO
    };

    if is_negative_sweep {
        face.clone()
    } else {
        face.reverse(services)
    }
}

fn sweep_cycle(
    bottom_cycle: &Cycle,
    bottom_surface: &Surface,
    color: Option<Color>,
    faces: &mut Vec<Handle<Face>>,
    path: Vector<3>,
    cache: &mut SweepCache,
    services: &mut Services,
) -> Handle<Cycle> {
    let swept_cycle = bottom_cycle.reverse(services).sweep_cycle(
        bottom_surface,
        color,
        path,
        cache,
        services,
    );

    faces.extend(
        swept_cycle
            .faces
            .into_iter()
            .map(|side_face| side_face.insert(services)),
    );

    swept_cycle.top_cycle.insert(services)
}
