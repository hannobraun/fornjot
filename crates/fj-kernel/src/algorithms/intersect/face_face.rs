use fj_interop::ext::ArrayExt;
use iter_fixed::IntoIteratorFixed;

use crate::{
    objects::{Curve, Face, Objects},
    services::Service,
    storage::Handle,
    validate::ValidationError,
};

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
    pub intersection_curves: [Handle<Curve>; 2],

    /// The interval of this intersection, in curve coordinates
    ///
    /// These curve coordinates apply to both intersection curves equally.
    pub intersection_intervals: CurveFaceIntersection,
}

impl FaceFaceIntersection {
    /// Compute the intersections between two faces
    pub fn compute(
        faces: [&Face; 2],
        objects: &mut Service<Objects>,
    ) -> Result<Option<Self>, ValidationError> {
        let surfaces = faces.map(|face| face.surface().clone());

        let intersection_curves =
            match SurfaceSurfaceIntersection::compute(surfaces, objects)? {
                Some(intersection) => intersection.intersection_curves,
                None => return Ok(None),
            };

        let curve_face_intersections = intersection_curves
            .each_ref_ext()
            .into_iter_fixed()
            .zip(faces)
            .map(|(curve, face)| CurveFaceIntersection::compute(curve, face))
            .collect::<[_; 2]>();

        let intersection_intervals = {
            let [a, b] = curve_face_intersections;
            a.merge(&b)
        };

        if intersection_intervals.is_empty() {
            return Ok(None);
        }

        Ok(Some(Self {
            intersection_curves,
            intersection_intervals,
        }))
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::ext::ArrayExt;
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::intersect::CurveFaceIntersection,
        builder::{CurveBuilder, FaceBuilder},
        insert::Insert,
        objects::{Face, Objects},
        partial::{HasPartial, PartialCurve},
        services::State,
        validate::ValidationError,
    };

    use super::FaceFaceIntersection;

    #[test]
    fn compute_no_intersection() -> anyhow::Result<()> {
        let mut objects = Objects::new().into_service();

        #[rustfmt::skip]
        let points = [
            [1., 1.],
            [2., 1.],
            [2., 2.],
            [1., 2.],
        ];
        let [a, b] = [objects.surfaces.xy_plane(), objects.surfaces.xz_plane()]
            .try_map_ext(|surface| {
                Face::partial()
                    .with_surface(surface)
                    .with_exterior_polygon_from_points(points)
                    .build(&mut objects)
            })?;

        let intersection =
            FaceFaceIntersection::compute([&a, &b], &mut objects)?;

        assert!(intersection.is_none());

        Ok(())
    }

    #[test]
    fn compute_one_intersection() -> anyhow::Result<()> {
        let mut objects = Objects::new().into_service();

        #[rustfmt::skip]
        let points = [
            [-1., -1.],
            [ 1., -1.],
            [ 1.,  1.],
            [-1.,  1.],
        ];
        let surfaces =
            [objects.surfaces.xy_plane(), objects.surfaces.xz_plane()];
        let [a, b] = surfaces.clone().try_map_ext(|surface| {
            Face::partial()
                .with_surface(surface)
                .with_exterior_polygon_from_points(points)
                .build(&mut objects)
        })?;

        let intersection =
            FaceFaceIntersection::compute([&a, &b], &mut objects)?;

        let expected_curves =
            surfaces.try_map_ext(|surface| -> Result<_, ValidationError> {
                let mut curve = PartialCurve {
                    surface: Some(surface),
                    ..Default::default()
                };
                curve.update_as_line_from_points([[0., 0.], [1., 0.]]);
                Ok(curve.build(&mut objects)?.insert(&mut objects))
            })?;
        let expected_intervals =
            CurveFaceIntersection::from_intervals([[[-1.], [1.]]]);
        assert_eq!(
            intersection,
            Some(FaceFaceIntersection {
                intersection_curves: expected_curves,
                intersection_intervals: expected_intervals
            })
        );
        Ok(())
    }
}
