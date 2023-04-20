use fj_math::Point;

use crate::{
    objects::{Face, Objects, Shell},
    operations::Insert,
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
        } = Face::triangle([a, b, c], [None, None, None], objects);
        let Triangle {
            face: face_abd,
            edges: [_, ad, db],
        } = Face::triangle([b, a, d], [Some(ab), None, None], objects);
        let Triangle {
            face: face_cad,
            edges: [_, _, cd],
        } = Face::triangle([d, a, c], [Some(ad), Some(ca), None], objects);
        let Triangle { face: face_bcd, .. } =
            Face::triangle([c, b, d], [Some(bc), Some(db), Some(cd)], objects);

        let faces = [face_abc, face_abd, face_cad, face_bcd]
            .map(|face| face.insert(objects));
        let shell = Shell::new(faces.clone());

        let [face_abc, face_abd, face_cad, face_bcd] = faces;

        Tetrahedron {
            shell,
            face_abc,
            face_abd,
            face_cad,
            face_bcd,
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

    /// The face formed by the points `a`, `b`, and `d`.
    pub face_abd: Handle<Face>,

    /// The face formed by the points `c`, `a`, and `d`.
    pub face_cad: Handle<Face>,

    /// The face formed by the points `b`, `c`, and `d`.
    pub face_bcd: Handle<Face>,
}
