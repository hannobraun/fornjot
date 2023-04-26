use fj_math::Point;

use crate::{
    objects::{Face, Shell},
    operations::{Insert, JoinCycle, UpdateFace},
    services::Services,
    storage::Handle,
};

use super::BuildFace;

/// Build a [`Shell`]
pub trait BuildShell {
    /// Build a tetrahedron from the provided points
    ///
    /// Accepts 4 points, naturally. For the purposes of the following
    /// discussion, let's call those `a`, `b`, `c`, and `d`, and assume that the
    /// order they are listed in here matches the order they are provided in
    /// within the array.
    ///
    /// Assumes that `a`, `b`, and `c` form a triangle in counter-clockwise
    /// order, when arranging the viewpoint such that it is on the opposite side
    /// of the triangle from `d`. If this assumption is met, the orientation of
    /// all faces of the tetrahedron will be valid, meaning their
    /// counter-clockwise sides are outside.
    ///
    /// # Implementation Note
    ///
    /// In principle, this method doesn't need to make assumptions about the
    /// order of the points provided. It could, given some extra effort, just
    /// build a correct tetrahedron, regardless of that order.
    fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        services: &mut Services,
    ) -> Tetrahedron {
        let [a, b, c, d] = points.map(Into::into);

        let abc = Face::triangle([a, b, c], services).face;
        let bad =
            Face::triangle([b, a, d], services)
                .face
                .update_exterior(|cycle| {
                    cycle
                        .join_to(
                            abc.exterior(),
                            0..=0,
                            0..=0,
                            &mut services.objects,
                        )
                        .insert(&mut services.objects)
                });
        let dac =
            Face::triangle([d, a, c], services)
                .face
                .update_exterior(|cycle| {
                    cycle
                        .join_to(
                            abc.exterior(),
                            1..=1,
                            2..=2,
                            &mut services.objects,
                        )
                        .join_to(
                            bad.exterior(),
                            0..=0,
                            1..=1,
                            &mut services.objects,
                        )
                        .insert(&mut services.objects)
                });
        let cbd =
            Face::triangle([c, b, d], services)
                .face
                .update_exterior(|cycle| {
                    cycle
                        .join_to(
                            abc.exterior(),
                            0..=0,
                            1..=1,
                            &mut services.objects,
                        )
                        .join_to(
                            bad.exterior(),
                            1..=1,
                            2..=2,
                            &mut services.objects,
                        )
                        .join_to(
                            dac.exterior(),
                            2..=2,
                            2..=2,
                            &mut services.objects,
                        )
                        .insert(&mut services.objects)
                });

        let faces =
            [abc, bad, dac, cbd].map(|face| face.insert(&mut services.objects));
        let shell = Shell::new(faces.clone());

        let [abc, bad, dac, cbd] = faces;

        Tetrahedron {
            shell,
            abc,
            bad,
            dac,
            cbd,
        }
    }
}

impl BuildShell for Shell {}

/// A tetrahedron
///
/// A tetrahedron is constructed from 4 points and has 4 faces. For the purpose
/// of naming the fields of this struct, the points are named `a`, `b`, `c`, and
/// `d`, in the order in which they are passed.
///
/// Returned by [`BuildShell::tetrahedron`].
pub struct Tetrahedron {
    /// The shell that forms the tetrahedron
    pub shell: Shell,

    /// The face formed by the points `a`, `b`, and `c`.
    pub abc: Handle<Face>,

    /// The face formed by the points `b`, `a`, and `d`.
    pub bad: Handle<Face>,

    /// The face formed by the points `d`, `a`, and `c`.
    pub dac: Handle<Face>,

    /// The face formed by the points `c`, `b`, and `d`.
    pub cbd: Handle<Face>,
}
