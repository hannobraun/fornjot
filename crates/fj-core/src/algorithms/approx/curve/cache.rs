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
    inner: BTreeMap<HandleWrapper<Curve>, CurveApprox>,
}

impl CurveApproxCache {
    /// Get an approximation from the cache
    pub fn get(
        &self,
        curve: &Handle<Curve>,
        boundary: &CurveBoundary<Point<1>>,
    ) -> CurveApprox {
        let curve = HandleWrapper::from(curve.clone());

        let mut approx = self.inner.get(&curve).cloned().unwrap_or_default();
        approx.make_subset(*boundary);

        // Approximations within the cache are stored in normalized form. If the
        // caller requests a non-normalized boundary, that means we need to
        // adjust the result before we return it.
        if !boundary.is_normalized() {
            approx.reverse();
        }

        approx
    }

    /// Insert an approximated segment of the curve into the cache
    pub fn insert(
        &mut self,
        curve: Handle<Curve>,
        mut new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let curve = HandleWrapper::from(curve);

        // Overlapping approximations need to result in the same points,
        // regardless of what direction those overlapping approximations happen
        // to have. To make sure this is always the case, we normalize each new
        // approximated segment before doing *anything* with it.
        new_segment.normalize();

        self.inner.entry(curve).or_default().merge(new_segment)
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

        // Also, the new segment should not have replaced the existing on in the
        // cache.
        let cached = cache.get(&curve, &boundary);
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![existing_segment]
            }
        );
    }

    #[test]
    fn insert_merge_non_overlapping_segments() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // Insert segments that are not overlapping. The segments themselves
        // don't need to be merged, but both segments still need to be merged
        // into the same curve approximation.
        //
        // Let's make sure they are out of order, to make sure that is taken
        // care of when doing the merge.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.75], [1.]]),
                points: vec![ApproxPoint::new([0.875], [0.875, 0.875, 0.875])],
            },
        );
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.], [0.25]]),
                points: vec![ApproxPoint::new([0.125], [0.125, 0.125, 0.125])],
            },
        );

        let cached = cache.get(&curve, &CurveBoundary::from([[0.], [1.]]));
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![
                    CurveApproxSegment {
                        boundary: CurveBoundary::from([[0.], [0.25]]),
                        points: vec![ApproxPoint::new(
                            [0.125],
                            [0.125, 0.125, 0.125]
                        )],
                    },
                    CurveApproxSegment {
                        boundary: CurveBoundary::from([[0.75], [1.]]),
                        points: vec![ApproxPoint::new(
                            [0.875],
                            [0.875, 0.875, 0.875]
                        )],
                    }
                ]
            }
        );
    }

    #[test]
    fn insert_merge_overlapping_segments() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        // Insert two non-overlapping segments to prepare for the actual test.
        // Make sure they are not normalized and out of order, to exercise that
        // functionality.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[1.5], [1.0]]),
                points: vec![
                    ApproxPoint::new([1.375], [1.375, 1.375, 1.375]),
                    ApproxPoint::new([1.125], [1.125, 1.125, 1.125]),
                ],
            },
        );
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.5], [0.]]),
                points: vec![
                    ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                    ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                ],
            },
        );

        // Now insert a third segment that overlaps both of them (touching
        // counts as overlapping). They should all get merged into a single
        // segment.
        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[1.0], [0.5]]),
                points: vec![
                    ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                    ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                ],
            },
        );

        let boundary = CurveBoundary::from([[0.], [1.5]]);
        let cached = cache.get(&curve, &boundary);
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![CurveApproxSegment {
                    boundary,
                    points: vec![
                        ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                        ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                        ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                        ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                        ApproxPoint::new([1.125], [1.125, 1.125, 1.125]),
                        ApproxPoint::new([1.375], [1.375, 1.375, 1.375]),
                    ],
                }]
            }
        );
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
            CurveApprox {
                segments: vec![segment]
            }
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
            CurveApprox {
                segments: vec![{
                    let mut segment = segment;
                    segment.reverse();
                    segment
                }]
            }
        );
    }

    #[test]
    fn get_partial_match_that_overlaps_start() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.], [1.]]),
                points: vec![
                    ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                    ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                    ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                    ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                ],
            }
            .clone(),
        );

        let cached = cache.get(&curve, &CurveBoundary::from([[-0.5], [0.5]]));
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![CurveApproxSegment {
                    boundary: CurveBoundary::from([[0.], [0.5]]),
                    points: vec![
                        ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                        ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                    ],
                }]
            }
        );
    }

    #[test]
    fn get_partial_match_that_overlaps_end() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.], [1.]]),
                points: vec![
                    ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                    ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                    ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                    ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                ],
            }
            .clone(),
        );

        let cached = cache.get(&curve, &CurveBoundary::from([[0.5], [1.5]]));
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![CurveApproxSegment {
                    boundary: CurveBoundary::from([[0.5], [1.0]]),
                    points: vec![
                        ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                        ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                    ],
                }]
            }
        );
    }

    #[test]
    fn get_partial_match_in_the_middle() {
        let mut services = Services::new();

        let mut cache = CurveApproxCache::default();
        let curve = Curve::new().insert(&mut services);

        cache.insert(
            curve.clone(),
            CurveApproxSegment {
                boundary: CurveBoundary::from([[0.], [1.]]),
                points: vec![
                    ApproxPoint::new([0.125], [0.125, 0.125, 0.125]),
                    ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                    ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                    ApproxPoint::new([0.875], [0.875, 0.875, 0.875]),
                ],
            }
            .clone(),
        );

        let cached = cache.get(&curve, &CurveBoundary::from([[0.25], [0.75]]));
        assert_eq!(
            cached,
            CurveApprox {
                segments: vec![CurveApproxSegment {
                    boundary: CurveBoundary::from([[0.25], [0.75]]),
                    points: vec![
                        ApproxPoint::new([0.375], [0.375, 0.375, 0.375]),
                        ApproxPoint::new([0.625], [0.625, 0.625, 0.625]),
                    ],
                }]
            }
        );
    }
}
