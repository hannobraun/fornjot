use crate::{
    handle::Handle,
    math::Plane,
    topology::{
        curve::Curve, face::Face, half_edge::HalfEdge, solid::Solid,
        surface::Surface,
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
        // Let's designate the two faces as "bottom" and "top", to make it
        // easier to talk about them and things related to them, in the
        // following code.
        let bottom = self;
        let top = other;

        assert_eq!(
            bottom.half_edges.len(),
            top.half_edges.len(),
            "Can only connect faces that have the same number of vertices.",
        );

        let connecting_faces = build_connecting_faces(&bottom, &top);

        Solid::new([bottom, top].into_iter().chain(connecting_faces))
    }
}

fn build_connecting_faces(bottom: &Face, top: &Face) -> Vec<Handle<Face>> {
    bottom
        .half_edges_with_end_vertex()
        .zip(top.half_edges_with_end_vertex())
        .map(|((bottom_a, b), (d, c))| {
            let is_internal = match [bottom_a.is_internal, d.is_internal] {
                [true, true] => true,
                [false, false] => false,
                _ => {
                    panic!(
                        "Trying to connect an internal half-edge of one \
                            face to an external half-edge of another"
                    );
                }
            };

            let surface = Handle::new(Surface {
                geometry: Box::new(Plane::from_points(
                    [&bottom_a.start, b, c].map(|vertex| vertex.point),
                )),
            });
            let face = Face::new(
                surface,
                [&bottom_a.start, b, c, &d.start].map(|vertex| {
                    Handle::new(HalfEdge {
                        curve: Handle::new(Curve {}),
                        start: vertex.clone(),
                        is_internal: false,
                    })
                }),
                is_internal,
            );
            Handle::new(face)
        })
        .collect::<Vec<_>>()
}
