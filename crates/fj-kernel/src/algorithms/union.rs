use std::collections::BTreeSet;

use crate::objects::Solid;

use super::intersection::{
    CurveFaceIntersectionList, SurfaceSurfaceIntersection,
};

/// Computes the shape that is the union of the two provided shapes
pub fn union(a: impl Into<Solid>, b: impl Into<Solid>) -> Solid {
    // TASK: Implement algorithm from "Boundary Representation Modelling
    //       Techniques", section 6.1.1 (pages 127 ff.).

    let a = a.into();
    let b = b.into();

    let mut faces = BTreeSet::new();

    // Check the faces of both shapes for intersections.
    for face_a in a.faces() {
        for face_b in b.faces() {
            // First step in determining the intersections between two faces:
            // Determine the intersection of their surfaces.
            let intersection = SurfaceSurfaceIntersection::compute(
                face_a.surface(),
                face_b.surface(),
            );

            let intersection = match intersection {
                Some(intersection) => intersection,
                None => {
                    // We're not getting an intersection curve, which means the
                    // surfaces either don't intersect (which can only mean
                    // they're parallel), or they are coincident.

                    // TASK: Handle surfaces being coincident.
                    continue;
                }
            };

            let [curve_a, curve_b] = intersection.local_intersection_curves;

            // Check the curve's intersections against the faces. The result of
            // this operation are the intersections between the faces.
            let intersections_a =
                CurveFaceIntersectionList::compute(&curve_a, face_a);
            let intersections_b =
                CurveFaceIntersectionList::compute(&curve_b, face_b);

            // Depending on which of the faces the two surface's intersection
            // curve intersects with, we can draw conclusions about which of
            // them we need to keep.
            //
            // If exactly one of the faces intersects, we know that this face is
            // part of the boundary of the union. We add it to `faces`.
            //
            // If we're not adding a face here, that is not a definite decision.
            // The face might still be added to `faces` in another loop
            // iteration, as part of a comparison with another face.

            match (intersections_a.is_empty(), intersections_b.is_empty()) {
                // Both faces intersect the intersection curve, which means they
                // intersect each other.
                (false, false) => {
                    // TASK: Figure out what this means. The faces need to be
                    //       cut to size somehow.
                }

                // Intersection curve intersects only one of the faces.
                //
                // TASK: This doesn't mean that that face should go into the
                //       union as-is. It means that some version of the face
                //       goes into the union, but if there's a proper
                //       intersection with another face, the face needs to be
                //       cut to size.
                //
                //       Add it to a list of candidates that still can be
                //       modified instead.
                (false, true) => {
                    faces.insert(face_a.clone());
                }
                (true, false) => {
                    faces.insert(face_b.clone());
                }

                // Intersection curve intersects none of the faces. This can
                // only happen if none of the faces is contained in a shell that
                // the other face is a part of. They must be part of distinct
                // solids, and both need to end up in the union.
                (true, true) => {
                    faces.insert(face_a.clone());
                    faces.insert(face_b.clone());
                }
            }

            // TASK: Implement.
            let _ = curve;
        }
    }

    Solid::new().with_faces(faces)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::{union, TransformObject},
        objects::Solid,
    };

    #[test]
    fn distinct() {
        let a = Solid::build()
            .cube_from_edge_length(1.)
            .translate([-1., -1., -1.]);
        let b = Solid::build()
            .cube_from_edge_length(1.)
            .translate([1., 1., 1.]);

        let mut all_faces = Vec::new();
        all_faces.extend(a.faces().cloned());
        all_faces.extend(b.faces().cloned());

        let union = union(a, b);

        assert_eq!(union, Solid::new().with_faces(all_faces));
    }

    #[test]
    fn a_contains_b() {
        let a = Solid::build().cube_from_edge_length(2.);
        let b = Solid::build().cube_from_edge_length(1.);

        let union = union(a.clone(), b);

        assert_eq!(union, a.into_solid());
    }

    #[test]
    fn b_contains_a() {
        let a = Solid::build().cube_from_edge_length(1.);
        let b = Solid::build().cube_from_edge_length(2.);

        let union = union(a, b.clone());

        assert_eq!(union, b.into_solid());
    }

    // TASK: intersecting, broken edges in a
    // TASK: intersection, broken edges in b
}
