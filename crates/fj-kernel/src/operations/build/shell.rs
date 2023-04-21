use fj_math::Point;

use crate::{
    objects::{Face, Objects, Shell},
    operations::{Insert, UpdateCycle, UpdateFace, UpdateHalfEdge},
    services::Service,
    storage::Handle,
};

use super::{BuildFace, Triangle};

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
        objects: &mut Service<Objects>,
    ) -> Tetrahedron {
        let [a, b, c, d] = points.map(Into::into);

        let Triangle {
            face: face_abc,
            edges: [ab, bc, ca],
            ..
        } = Face::triangle([a, b, c], objects);
        let Triangle {
            face: face_bad,
            edges: [ba, ad, db],
            ..
        } = Face::triangle([b, a, d], objects);
        let Triangle {
            face: face_dac,
            edges: [da, ac, cd],
            ..
        } = Face::triangle([d, a, c], objects);
        let Triangle {
            face: face_cbd,
            edges: [cb, bd, dc],
            ..
        } = Face::triangle([c, b, d], objects);

        let face_bad = face_bad.update_exterior(|cycle| {
            let ba_joined = ba
                .replace_global_form(ab.global_form().clone())
                .insert(objects);

            cycle.replace_half_edge(&ba, ba_joined).insert(objects)
        });
        let face_dac = face_dac.update_exterior(|cycle| {
            let da_joined = da
                .replace_global_form(ad.global_form().clone())
                .insert(objects);
            let ac_joined = ac
                .replace_global_form(ca.global_form().clone())
                .insert(objects);

            cycle
                .replace_half_edge(&da, da_joined)
                .replace_half_edge(&ac, ac_joined)
                .insert(objects)
        });
        let face_cbd = face_cbd.update_exterior(|cycle| {
            let cb_joined = cb
                .replace_global_form(bc.global_form().clone())
                .insert(objects);
            let bd_joined = bd
                .replace_global_form(db.global_form().clone())
                .insert(objects);
            let dc_joined = dc
                .replace_global_form(cd.global_form().clone())
                .insert(objects);

            cycle
                .replace_half_edge(&cb, cb_joined)
                .replace_half_edge(&bd, bd_joined)
                .replace_half_edge(&dc, dc_joined)
                .insert(objects)
        });

        let faces = [face_abc, face_bad, face_dac, face_cbd]
            .map(|face| face.insert(objects));
        let shell = Shell::new(faces.clone());

        let [face_abc, face_bad, face_dac, face_cbd] = faces;

        Tetrahedron {
            shell,
            face_abc,
            face_bad,
            face_dac,
            face_cbd,
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
    pub face_abc: Handle<Face>,

    /// The face formed by the points `b`, `a`, and `d`.
    pub face_bad: Handle<Face>,

    /// The face formed by the points `d`, `a`, and `c`.
    pub face_dac: Handle<Face>,

    /// The face formed by the points `c`, `b`, and `d`.
    pub face_cbd: Handle<Face>,
}
