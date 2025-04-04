use crate::{
    handle::Handle,
    math::Plane,
    topology::{
        curve::Curve,
        face::{Face, HalfEdgeWithEndVertex},
        half_edge::HalfEdge,
        solid::Solid,
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

        let connecting_faces = build_connecting_faces([&bottom, &top]);

        Solid::new([bottom, top].into_iter().chain(connecting_faces))
    }
}

fn build_connecting_faces([bottom, top]: [&Face; 2]) -> Vec<Handle<Face>> {
    bottom
        .half_edges_with_end_vertex()
        .zip(top.half_edges_with_end_vertex())
        .map(|(bottom, top)| build_single_connecting_face([bottom, top]))
        .collect::<Vec<_>>()
}

fn build_single_connecting_face(
    [bottom, top]: [HalfEdgeWithEndVertex; 2],
) -> Handle<Face> {
    let is_internal =
        match [bottom.half_edge.is_internal, top.half_edge.is_internal] {
            [true, true] => true,
            [false, false] => false,
            _ => {
                panic!(
                    "Trying to connect an internal half-edge of one face to an \
                    external half-edge of another."
                );
            }
        };

    // Order the vertices in a way that makes sense, when building the
    // half-edges of the connecting face.
    let a = &bottom.half_edge.start;
    let b = bottom.end_vertex;
    let c = top.end_vertex;
    let d = &top.half_edge.start;

    let surface = Handle::new(Surface {
        geometry: Box::new(Plane::from_points(
            [a, b, d].map(|vertex| vertex.point),
        )),
    });
    let half_edges = [a, b, c, d].map(|vertex| {
        Handle::new(HalfEdge {
            curve: Handle::new(Curve {}),
            start: vertex.clone(),
            is_internal: false,
        })
    });

    let face = Face::new(surface, half_edges, is_internal);
    Handle::new(face)
}
