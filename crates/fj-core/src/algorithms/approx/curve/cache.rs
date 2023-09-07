use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::Curve,
    storage::{Handle, HandleWrapper},
};

use super::{CurveApprox, CurveApproxSegment};

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner:
        BTreeMap<(HandleWrapper<Curve>, CurveBoundary<Point<1>>), CurveApprox>,
}

impl CurveApproxCache {
    /// Get an approximation from the cache
    pub fn get(
        &self,
        curve: &Handle<Curve>,
        boundary: &CurveBoundary<Point<1>>,
    ) -> Option<CurveApprox> {
        let curve = HandleWrapper::from(curve.clone());

        // Approximations within the cache are all stored in normalized form. If
        // the caller requests a non-normalized boundary, that means we need to
        // adjust the result before we return it, so let's remember whether the
        // normalization resulted in a reversal.
        let was_already_normalized = boundary.is_normalized();
        let normalized_boundary = boundary.normalize();

        self.inner.get(&(curve, normalized_boundary)).cloned().map(
            |mut approx| {
                if !was_already_normalized {
                    approx.reverse();
                }

                approx
            },
        )
    }

    /// Insert an approximated segment of the curve into the cache
    pub fn insert(
        &mut self,
        curve: Handle<Curve>,
        mut new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let curve = HandleWrapper::from(curve);

        new_segment.normalize();

        // We assume that approximated curve segments never overlap, so we don't
        // have to do any merging of this segment with a possibly existing
        // approximation for this curve.
        //
        // For now, this is a valid assumption, as it matches the uses of this
        // method, due to documented limitations elsewhere in the system.
        let approx = CurveApprox {
            segments: vec![new_segment.clone()],
        };

        self.inner
            .insert((curve, new_segment.boundary), approx)
            .map(|approx| {
                let mut segments = approx.segments.into_iter();
                let segment = segments
                    .next()
                    .expect("Empty approximations should not exist in cache");
                assert!(
                    segments.next().is_none(),
                    "Cached approximations should have exactly 1 segment"
                );
                segment
            })
            .unwrap_or(new_segment)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        algorithms::approx::{
            curve::{CurveApprox, CurveApproxCache, CurveApproxSegment},
            ApproxPoint,
        },
        geometry::CurveBoundary,
        objects::Curve,
        operations::Insert,
        services::Services,
    };

    #[test]
    fn insert_curve_already_exists_but_no_segment_merge_necessary() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // An approximation of our curve already exists.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[2.], [3.]]),
                points: vec![
                    ApproxPoint::new([2.25], [2.25, 2.25, 2.25]),
                    ApproxPoint::new([2.75], [2.75, 2.75, 2.75]),
                ],
            },
        );

        // Here's another approximated segment for the same curve, but i doesn't
        // overlap with the already existing one.
        let boundary = CurveBoundary::from([[0.], [1.]]);
        let segment = CurveApproxSegment {
            boundary,
            points: vec![
                ApproxPoint::new([0.25], [0.25, 0.25, 0.25]),
                ApproxPoint::new([0.75], [0.75, 0.75, 0.75]),
            ],
        };

        // When inserting the second segment, we expect to get it back
        // unchanged.
        let inserted = cache.insert(curve.clone(), segment.clone());
        assert_eq!(inserted, segment);
    }

    #[test]
    fn insert_congruent_segment_already_exists() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // An approximation of our curve already exists.
        let boundary = CurveBoundary::from([[0.], [1.]]);
        let existing_segment = CurveApproxSegment {
            boundary,
            points: vec![
                ApproxPoint::new([0.25], [0.25, 0.25, 0.25]),
                ApproxPoint::new([0.75], [0.75, 0.75, 0.75]),
            ],
        };
        cache.insert(curve.clone(), existing_segment.clone());

        // Here's another approximated segment for the same curve that is
        // congruent with the existing one.
        let new_segment = CurveApproxSegment {
            boundary,
            points: vec![
                ApproxPoint::new([0.24], [0.24, 0.24, 0.24]),
                ApproxPoint::new([0.76], [0.76, 0.76, 0.76]),
            ],
        };

        // When inserting the second segment, we expect to get the original one
        // back.
        let inserted = cache.insert(curve.clone(), new_segment);
        assert_eq!(inserted, existing_segment);
    }

    #[test]
    fn get_exact_match() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // An approximation of our curve already exists.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[2.], [3.]]),
                points: vec![
                    ApproxPoint::new([2.25], [2.25, 2.25, 2.25]),
                    ApproxPoint::new([2.75], [2.75, 2.75, 2.75]),
                ],
            },
        );

        // Here's a second segment that doesn't overlap the existing one.
        let boundary = CurveBoundary::from([[0.], [1.]]);
        let segment = CurveApproxSegment {
            boundary,
            points: vec![
                ApproxPoint::new([0.25], [0.25, 0.25, 0.25]),
                ApproxPoint::new([0.75], [0.75, 0.75, 0.75]),
            ],
        };
        cache.insert(curve.clone(), segment.clone());

        // When asking for an approximation with the same boundary as the second
        // segment we added, we expect to get it back exactly.
        let cached = cache.get(&curve, &boundary);
        assert_eq!(
            cached,
            Some(CurveApprox {
                segments: vec![segment]
            })
        );
    }

    #[test]
    fn get_exact_match_except_reversed() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // An approximation of our curve already exists.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[2.], [3.]]),
                points: vec![
                    ApproxPoint::new([2.25], [2.25, 2.25, 2.25]),
                    ApproxPoint::new([2.75], [2.75, 2.75, 2.75]),
                ],
            },
        );

        // Here's a second segment that doesn't overlap the existing one.
        let boundary = CurveBoundary::from([[0.], [1.]]);
        let segment = CurveApproxSegment {
            points: vec![
                ApproxPoint::new([0.25], [0.25, 0.25, 0.25]),
                ApproxPoint::new([0.75], [0.75, 0.75, 0.75]),
            ],
            boundary,
        };
        cache.insert(curve.clone(), segment.clone());

        // When asking for an approximation with the same boundary of the second
        // segment we added but reversed, we expect to get back the segment, but
        // reversed.
        let cached = cache.get(&curve, &boundary.reverse());
        assert_eq!(
            cached,
            Some(CurveApprox {
                segments: vec![{
                    let mut segment = segment;
                    segment.reverse();
                    segment
                }]
            })
        );
    }
}
