use std::collections::HashSet;

use decorum::R64;
use parry3d_f64::shape::Segment;

use crate::math::Point;

/// An approximation of an edge, multiple edges, or a face
pub struct Approximation {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: Vec<Point<3>>,

    /// Segments that approximate edges
    ///
    /// Every approximation will involve edges, typically, and these are
    /// approximated by these segments.
    ///
    /// All the points of these segments will also be available in the `points`
    /// field of this struct. This can be verified by calling
    /// [`Approximation::validate`].
    pub segments: Vec<Segment>,
}

impl Approximation {
    /// Validate the approximation
    ///
    /// Returns an `Err(ValidationError)`, if the validation is not valid. See
    /// [`ValidationError`] for the ways that the approximation can be invalid.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut duplicate_points = Vec::new();
        let mut duplicate_segments = Vec::new();
        let mut invalid_segments = Vec::new();
        let mut segments_with_invalid_points = Vec::new();

        // Verify that there are no duplicate points
        let mut points = HashSet::new();
        for &point in &self.points {
            let point_r64 = point_to_r64(point);

            if points.contains(&point_r64) {
                duplicate_points.push(point);
            }

            points.insert(point_r64);
        }

        let mut segments = HashSet::new();
        for &segment @ Segment { a, b } in &self.segments {
            // Verify that there are no duplicate segments
            let ab = [point_to_r64(a), point_to_r64(b)];
            let ba = [point_to_r64(b), point_to_r64(a)];
            if segments.contains(&ab) {
                duplicate_segments.push(segment);
            }
            segments.insert(ab);
            segments.insert(ba);

            // Verify that segments are actually segments
            if a == b {
                invalid_segments.push(segment);
            }

            // Verify that segments refer to valid points
            if !(self.points.contains(&a) && self.points.contains(&b)) {
                segments_with_invalid_points.push(segment);
            }
        }

        if !(duplicate_points.is_empty()
            && duplicate_segments.is_empty()
            && invalid_segments.is_empty()
            && segments_with_invalid_points.is_empty())
        {
            return Err(ValidationError {
                duplicate_points,
                duplicate_segments,
                invalid_segments,
                segments_with_invalid_points,
            });
        }

        Ok(())
    }
}

/// Error returned by [`Approximation::validate`]
#[derive(Debug)]
pub struct ValidationError {
    /// Points that are duplicated
    pub duplicate_points: Vec<Point<3>>,

    /// Segments that are duplicated
    pub duplicate_segments: Vec<Segment>,

    /// Segments that have two equal points
    pub invalid_segments: Vec<Segment>,

    /// Segments that do not refer to points from the approximation
    pub segments_with_invalid_points: Vec<Segment>,
}

fn point_to_r64(point: Point<3>) -> [R64; 3] {
    [point.x.into(), point.y.into(), point.z.into()]
}

#[cfg(test)]
mod tests {
    use nalgebra::point;
    use parry3d_f64::shape::Segment;

    use super::Approximation;

    #[test]
    fn test_validate() {
        let a = point![0., 1., 2.];
        let b = point![1., 2., 3.];
        let c = point![3., 5., 8.];

        let valid = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }],
        };
        assert!(valid.validate().is_ok());

        let duplicate_points = Approximation {
            points: vec![a, b, c, b],
            segments: vec![Segment { a, b }],
        };
        assert!(duplicate_points.validate().is_err());

        let duplicate_segments = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }, Segment { a, b }],
        };
        assert!(duplicate_segments.validate().is_err());

        let duplicate_segments_inverted = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }, Segment { a: b, b: a }],
        };
        assert!(duplicate_segments_inverted.validate().is_err());

        let invalid_segment = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b: a }],
        };
        assert!(invalid_segment.validate().is_err());

        let segment_with_invalid_point = Approximation {
            points: vec![a, c],
            segments: vec![Segment { a, b }],
        };
        assert!(segment_with_invalid_point.validate().is_err());
    }
}
