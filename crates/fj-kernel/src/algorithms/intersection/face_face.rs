use crate::objects::{Curve, Face};

use super::{CurveFaceIntersection, SurfaceSurfaceIntersection};

/// An intersection between two faces
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FaceFaceIntersection {
    /// The intersection curves
    ///
    /// These curves correspond to the input faces, each being the local
    /// representation of the intersection on the respective face's surface.
    ///
    /// They both represent the same global curve.
    pub intersection_curves: [Curve; 2],

    /// The interval of this intersection, in curve coordinates
    ///
    /// These curve coordinates apply to both intersection curves equally.
    pub intersection_intervals: CurveFaceIntersection,
}

impl FaceFaceIntersection {
    /// Compute the intersections between two faces
    pub fn compute(faces: [&Face; 2]) -> Option<Self> {
        let surfaces = faces.map(|face| face.surface());

        let intersection_curves =
            SurfaceSurfaceIntersection::compute(surfaces)?.intersection_curves;

        // Can be cleaned up, once `zip` is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.zip
        let curve_face_intersections = {
            let [curve_a, curve_b] = intersection_curves;
            let [face_a, face_b] = faces;

            [(curve_a, face_a), (curve_b, face_b)].map(|(curve, face)| {
                CurveFaceIntersection::compute(&curve, face)
            })
        };

        let intersection_intervals = {
            let [a, b] = curve_face_intersections;
            a.merge(&b)
        };

        if intersection_intervals.is_empty() {
            return None;
        }

        Some(Self {
            intersection_curves,
            intersection_intervals,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::intersection::CurveFaceIntersection,
        objects::{Curve, Face, Surface},
    };

    use super::FaceFaceIntersection;

    #[test]
    fn compute_no_intersection() {
        #[rustfmt::skip]
        let points = [
            [1., 1.],
            [2., 1.],
            [2., 2.],
            [1., 2.],
        ];
        let surfaces = [Surface::xy_plane(), Surface::xz_plane()];
        let [a, b] = surfaces.map(|surface| {
            Face::build(surface).polygon_from_points(points).into_face()
        });

        let intersection = FaceFaceIntersection::compute([&a, &b]);

        assert!(intersection.is_none());
    }

    #[test]
    fn compute_one_intersection() {
        #[rustfmt::skip]
        let points = [
            [-1., -1.],
            [ 1., -1.],
            [ 1.,  1.],
            [-1.,  1.],
        ];
        let surfaces = [Surface::xy_plane(), Surface::xz_plane()];
        let [a, b] = surfaces.map(|surface| {
            Face::build(surface).polygon_from_points(points).into_face()
        });

        let intersection = FaceFaceIntersection::compute([&a, &b]);

        let expected_curves = surfaces.map(|surface| {
            Curve::build(surface).line_from_points([[0., 0.], [1., 0.]])
        });
        let expected_intervals =
            CurveFaceIntersection::from_intervals([[[-1.], [1.]]]);
        assert_eq!(
            intersection,
            Some(FaceFaceIntersection {
                intersection_curves: expected_curves,
                intersection_intervals: expected_intervals
            })
        );
    }
}
