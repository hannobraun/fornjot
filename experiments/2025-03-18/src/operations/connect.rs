use itertools::Itertools;

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
        check_that_bottom_and_top_curves_are_shared(
            [&bottom, &top],
            &connecting_faces,
        );

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

    let surface = Handle::new(Surface {
        geometry: Box::new(Plane::from_points(
            [
                &bottom.half_edge.start,
                bottom.end_vertex,
                &top.half_edge.start,
            ]
            .map(|vertex| vertex.point),
        )),
    });

    // Order the vertices in a way that makes sense when building the half-edges
    // of the connecting face, and pair them with the curve required for the
    // respective half-edge.
    let a = (&bottom.half_edge.start, Some(&bottom.half_edge.curve));
    let b = (bottom.end_vertex, None);
    let c = (top.end_vertex, Some(&top.half_edge.curve));
    let d = (&top.half_edge.start, None);

    let half_edges = [a, b, c, d].map(|(vertex, maybe_curve)| {
        let curve = maybe_curve
            .cloned()
            .unwrap_or_else(|| Handle::new(Curve {}));

        Handle::new(HalfEdge {
            curve,
            start: vertex.clone(),
            is_internal: false,
        })
    });

    let face = Face::new(surface, half_edges, is_internal);
    Handle::new(face)
}

fn check_that_bottom_and_top_curves_are_shared(
    [bottom, top]: [&Face; 2],
    connecting_faces: &[Handle<Face>],
) {
    bottom
        .half_edges
        .iter()
        .zip(&top.half_edges)
        .zip(connecting_faces)
        .for_each(|((bottom, top), connecting)| {
            let Some([connecting_bottom, _, connecting_top, _]) =
                connecting.half_edges.iter().collect_array()
            else {
                unreachable!(
                    "Created connecting with exactly four half-edges."
                );
            };

            assert_eq!(bottom.curve, connecting_bottom.curve);
            assert_eq!(top.curve, connecting_top.curve);
        });
}
