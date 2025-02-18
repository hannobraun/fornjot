use crate::{
    math::Plane,
    operation::{Handle, OperationOutput},
};

use super::{face::Face, solid::Solid};

pub trait ConnectExt {
    /// # Connect two faces by creating a side wall of faces from their vertices
    ///
    /// ## Panics
    ///
    /// Panics, if the two faces provided do not have the same number of
    /// vertices.
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
    fn connect(self, other: Self) -> Handle<Solid>;
}

impl ConnectExt for Handle<Face> {
    fn connect(self, other: Self) -> Handle<Solid> {
        assert_eq!(
            self.output().vertices().count(),
            other.output().vertices().count(),
            "Can only connect faces that have the same number of vertices.",
        );

        let side_faces = self
            .output()
            .half_edges()
            .zip(other.output().half_edges())
            .map(|([q, r], [t, s])| {
                let surface = Plane::from_points(
                    [q, r, s].map(|vertex| vertex.output().point),
                );
                let face = Face::new(
                    surface,
                    [q, r, s, t].map(|vertex| vertex.clone()),
                );
                Handle::new(face)
            })
            .collect::<Vec<_>>();

        let output = Solid::new([self, other].into_iter().chain(side_faces));
        Handle::new(output)
    }
}
