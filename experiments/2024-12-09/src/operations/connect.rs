use crate::{
    math::Plane,
    object::Handle,
    topology::{
        face::Face, half_edge::HalfEdge, solid::Solid, surface::Surface,
    },
};

pub trait ConnectExt {
    /// # Connect two faces by creating a side wall of faces from their vertices
    ///
    /// ## Panics
    ///
    /// Panics, if the two faces provided do not have the same number of
    /// half-edges.
    ///
    /// Panics, if an internal half-edge of one face would connect to an
    /// external half-edge of the other.
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
    fn connect(self, other: Self) -> Solid;
}

impl ConnectExt for Handle<Face> {
    fn connect(self, other: Self) -> Solid {
        assert_eq!(
            self.half_edges.len(),
            other.half_edges.len(),
            "Can only connect faces that have the same number of vertices.",
        );

        let side_faces = self
            .half_edges_with_end_vertex()
            .zip(other.half_edges_with_end_vertex())
            .map(|((q, r), (t, s))| {
                let is_internal = match [q.is_internal, t.is_internal] {
                    [true, true] => true,
                    [false, false] => false,
                    _ => {
                        panic!(
                            "Trying to connect an internal half-edge of one \
                            face to an external half-edge of another"
                        );
                    }
                };

                let surface = Surface {
                    geometry: Plane::from_points(
                        [&q.start, r, s].map(|vertex| vertex.point),
                    ),
                };
                let face = Face::new(
                    surface,
                    [&q.start, r, s, &t.start].map(|vertex| {
                        Handle::new(HalfEdge {
                            start: vertex.clone(),
                            is_internal: false,
                        })
                    }),
                    is_internal,
                );
                Handle::new(face)
            })
            .collect::<Vec<_>>();

        Solid::new([self, other].into_iter().chain(side_faces))
    }
}
