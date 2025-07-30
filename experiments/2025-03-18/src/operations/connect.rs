use fj_interop::Tolerance;
use itertools::Itertools;

use crate::{
    geometry::{AnchoredCurve, FloatingCurve, SweptCurve},
    handle::Handle,
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
    fn connect(
        self,
        other: Self,
        with: FloatingCurve,
        tolerance: impl Into<Tolerance>,
    ) -> Solid;
}

impl ConnectExt for Handle<Face> {
    fn connect(
        self,
        other: Self,
        with: FloatingCurve,
        tolerance: impl Into<Tolerance>,
    ) -> Solid {
        let tolerance = tolerance.into();

        // Let's designate the two faces as "bottom" and "top", to make it
        // easier to talk about them and things related to them, in the
        // following code.
        //
        // These designations might or might not correspond to "top" and "down"
        // in 3D space, depending on the way the connecting curve points.
        let bottom = self;
        let top = other;

        assert_eq!(
            bottom.half_edges.len(),
            top.half_edges.len(),
            "Can only connect faces that have the same number of vertices.",
        );

        let connecting_faces = build_connecting_faces([&bottom, &top], with);

        // This is doing some checks, to make sure that the faces have been
        // connected correctly. There are other ways to do this. For now, this
        // is probably the best one, based on the following considerations:
        //
        // 1. These checks could also be done in a unit test, but doing it here
        //    provides a stronger guarantee. It makes sure that the invariants
        //    are true for all data that comes through here, not just a simple
        //    example that a unit test would construct.
        // 2. The invariants that are checked here should, in a generalized
        //    form, be true for all solids, not just the one constructed here.
        //    But so far, there's no infrastructure for this kind of validation.
        //
        // Long-term, such validation infrastructure should exist, and then we
        // can generalize these checks and move them there.
        check_that_bottom_and_top_curves_are_shared(
            [&bottom, &top],
            &connecting_faces,
        );
        check_that_connecting_curves_are_shared(&connecting_faces);
        check_that_connecting_curves_actually_connect_vertices(
            [&bottom, &top],
            &connecting_faces,
            tolerance,
        );

        Solid::new([bottom, top].into_iter().chain(connecting_faces))
    }
}

fn build_connecting_faces(
    [bottom, top]: [&Face; 2],
    connecting_curve: FloatingCurve,
) -> Vec<Handle<Face>> {
    let connecting_curves = build_connecting_curves(bottom, connecting_curve);

    connecting_curves
        .into_iter()
        .circular_tuple_windows()
        .zip(bottom.half_edges_with_end_vertex())
        .zip(top.half_edges_with_end_vertex())
        .map(|(((curve_down, curve_up), bottom), top)| {
            build_single_connecting_face([bottom, top], [curve_down, curve_up])
        })
        .collect::<Vec<_>>()
}

fn build_connecting_curves(
    bottom: &Face,
    connecting_curve: FloatingCurve,
) -> Vec<Handle<Curve>> {
    let bottom_vertices =
        bottom.half_edges.iter().map(|half_edge| &half_edge.start);

    bottom_vertices
        .map(|bottom| {
            let curve = Curve {
                geometry: AnchoredCurve {
                    origin: bottom.point,
                    floating: connecting_curve.clone(),
                },
            };

            Handle::new(curve)
        })
        .collect::<Vec<_>>()
}

fn build_single_connecting_face(
    [bottom, top]: [HalfEdgeWithEndVertex; 2],
    [curve_down, curve_up]: [Handle<Curve>; 2],
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
        geometry: Box::new(SweptCurve {
            u: bottom.half_edge.curve.geometry.clone(),
            v: curve_up.geometry.floating.flip(),
        }),
    });

    // Order the vertices in a way that makes sense when building the half-edges
    // of the connecting face, and pair them with the curve required for the
    // respective half-edge.
    let a = (&bottom.half_edge.start, &bottom.half_edge.curve);
    let b = (bottom.end_vertex, &curve_up);
    let c = (top.end_vertex, &top.half_edge.curve);
    let d = (&top.half_edge.start, &curve_down);

    let half_edges = [a, b, c, d].map(|(vertex, curve)| {
        Handle::new(HalfEdge {
            curve: curve.clone(),
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
                    "Created connecting faces with exactly four half-edges."
                );
            };

            assert_eq!(bottom.curve, connecting_bottom.curve);
            assert_eq!(top.curve, connecting_top.curve);
        });
}

fn check_that_connecting_curves_are_shared(connecting_faces: &[Handle<Face>]) {
    connecting_faces
        .iter()
        .circular_tuple_windows()
        .for_each(|(a, b)| {
            let [Some([_, a, _, _]), Some([_, _, _, b])] =
                [a, b].map(|face| face.half_edges.iter().collect_array())
            else {
                unreachable!(
                    "Created connecting faces with exactly four half-edges."
                );
            };

            assert_eq!(a.curve, b.curve);
        });
}

fn check_that_connecting_curves_actually_connect_vertices(
    [bottom, top]: [&Face; 2],
    connecting_faces: &[Handle<Face>],
    _: Tolerance,
) {
    let [bottom_vertices, top_vertices] = [bottom, top]
        .map(|face| face.half_edges.iter().map(|half_edge| &half_edge.start));
    let connecting_curves = connecting_faces.iter().map(|face| {
        let Some([_, _, _, half_edge]) = face.half_edges.iter().collect_array()
        else {
            unreachable!(
                "Created connecting faces with exactly four half-edges."
            );
        };

        &half_edge.curve
    });

    bottom_vertices
        .zip(top_vertices)
        .zip(connecting_curves)
        .for_each(|((bottom_vertex, top_vertex), connecting_curve)| {
            assert_eq!(bottom_vertex.point, connecting_curve.geometry.origin);

            // We also need to check that `top_vertex` is on the curve.
            let _ = top_vertex;
        });
}
