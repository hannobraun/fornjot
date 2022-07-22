use std::vec;

use fj_math::{Scalar, Segment};
use parry2d_f64::query::{Ray, RayCast};

use crate::objects::{Curve, Face};

/// The intersections between a [`Curve`] and a [`Face`], in curve coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveFaceIntersectionList {
    intervals: Vec<CurveFaceIntersection>,
}

impl CurveFaceIntersectionList {
    /// Create a new instance from the intersection intervals
    ///
    /// This method is useful for test code.
    pub fn from_intervals(
        intervals: impl IntoIterator<Item = [impl Into<Scalar>; 2]>,
    ) -> Self {
        let intervals = intervals
            .into_iter()
            .map(|interval| interval.map(Into::into))
            .collect();
        Self { intervals }
    }

    /// Compute the intersections between a [`Curve`] and a [`Face`]
    pub fn compute(curve: &Curve<2>, face: &Face) -> Self {
        let line = match curve {
            Curve::Line(line) => line,
            _ => todo!("Curve-face intersection only supports lines"),
        };

        let face_as_polygon = face
            .exteriors()
            .chain(face.interiors())
            .flat_map(|cycle| {
                let edges: Vec<_> = cycle.edges().cloned().collect();
                edges
            })
            .map(|edge| {
                let line = match edge.curve().local_form() {
                    Curve::Line(line) => line,
                    _ => {
                        todo!("Curve-face intersection only supports polygons")
                    }
                };

                let vertices = match edge.vertices().get() {
                    Some(vertices) => vertices.map(|&vertex| vertex),
                    None => todo!(
                        "Curve-face intersection does not support faces with \
                    continuous edges"
                    ),
                };

                (*line, vertices)
            });

        let mut intersections = Vec::new();

        for (edge_line, vertices) in face_as_polygon {
            let vertices = vertices.map(|vertex| {
                edge_line.point_from_line_coords(vertex.position())
            });
            let segment = Segment::from_points(vertices);

            let ray = Ray {
                origin: line.origin.to_na(),
                dir: line.direction.to_na(),
            };
            let ray_inv = Ray {
                origin: line.origin.to_na(),
                dir: -line.direction.to_na(),
            };

            let result =
                segment
                    .to_parry()
                    .cast_local_ray(&ray, f64::INFINITY, false);
            let result_inv = segment.to_parry().cast_local_ray(
                &ray_inv,
                f64::INFINITY,
                false,
            );

            if let Some(result) = result {
                intersections.push(Scalar::from(result));
            }
            if let Some(result_inv) = result_inv {
                intersections.push(-Scalar::from(result_inv));
            }
        }

        assert!(intersections.len() % 2 == 0);

        intersections.sort();

        // Can be cleaned up, once `array_chunks` is stable:
        // https://doc.rust-lang.org/std/primitive.slice.html#method.array_chunks
        let intervals = intersections
            .chunks(2)
            .map(|chunk| {
                // Can't panic, as we passed `2` to `windows`.
                [chunk[0], chunk[1]]
            })
            .collect();

        CurveFaceIntersectionList { intervals }
    }

    /// Merge this intersection list with another
    ///
    /// The merged list will contain all overlaps of the intervals from the two
    /// other lists.
    pub fn merge(&self, other: &Self) -> Self {
        let mut self_ = self.intervals.iter().copied();
        let mut other = other.intervals.iter().copied();

        let mut next_self = self_.next();
        let mut next_other = other.next();

        let mut intervals = Vec::new();

        while let (
            Some([self_start, self_end]),
            Some([other_start, other_end]),
        ) = (next_self, next_other)
        {
            // If we're starting another loop iteration, we have another
            // interval available from both `self` and `other` each. Only if
            // that's the case, is there a chance for an overlap.

            // Build the overlap of the two next intervals, by comparing them.
            // At this point we don't know yet, if this is a valid interval.
            let overlap_start = self_start.max(other_start);
            let overlap_end = self_end.min(other_end);

            if overlap_start < overlap_end {
                // This is indeed a valid overlap. Add it to our list of
                // results.
                intervals.push([overlap_start, overlap_end]);
            }

            // Only if the end of the overlap interval has overtaken one of the
            // input ones are we done with it. An input interval that hasn't
            // been overtaken by the overlap, could still overlap with another
            // interval.
            if self_end <= overlap_end {
                // Current interval from `self` has been overtaken. Let's grab
                // the next one.
                next_self = self_.next();
            }
            if other_end <= overlap_end {
                // Current interval from `other` has been overtaken. Let's grab
                // the next one.
                next_other = other.next();
            }
        }

        Self { intervals }
    }

    /// Indicate whether the intersection list is empty
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
}

impl IntoIterator for CurveFaceIntersectionList {
    type Item = CurveFaceIntersection;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intervals.into_iter()
    }
}

/// An intersection between a curve and a face, in curve coordinates
pub type CurveFaceIntersection = [Scalar; 2];

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};

    use crate::objects::{Curve, Cycle, Face, Surface};

    use super::CurveFaceIntersectionList;

    #[test]
    fn compute() {
        let curve = Curve::Line(Line {
            origin: Point::from([-3., 0.]),
            direction: Vector::from([1., 0.]),
        });

        #[rustfmt::skip]
        let exterior = [
            [-2., -2.],
            [ 2., -2.],
            [ 2.,  2.],
            [-2.,  2.],
        ];
        #[rustfmt::skip]
        let interior = [
            [-1., -1.],
            [ 1., -1.],
            [ 1.,  1.],
            [-1.,  1.],
        ];

        let surface = Surface::xy_plane();
        let face = Face::builder(surface)
            .with_exterior(Cycle::polygon_from_points(&surface, exterior))
            .with_interior(Cycle::polygon_from_points(&surface, interior))
            .build();

        let expected =
            CurveFaceIntersectionList::from_intervals([[1., 2.], [4., 5.]]);
        assert_eq!(CurveFaceIntersectionList::compute(&curve, &face), expected);
    }

    #[test]
    fn merge() {
        let a = CurveFaceIntersectionList::from_intervals([
            [0., 1.],   // 1: `a` and `b` are equal
            [2., 5.],   // 2: `a` contains `b`
            [7., 8.],   // 3: `b` contains `a`
            [9., 11.],  // 4: overlap; `a` is left
            [14., 16.], // 5: overlap; `a` is right
            [18., 21.], // 6: one of `a` partially overlaps two of `b`
            [23., 25.], // 7: two of `a` partially overlap one of `b`
            [26., 28.], // 7
            [31., 35.], // 8: one of `a` overlaps two of `b`; partial/complete
            [36., 38.], // 9: two of `a` overlap one of `b`; partial/complete
            [39., 40.], // 9
            [41., 45.], // 10: one of `a` overlaps two of `b`; complete/partial
            [48., 49.], // 11: two of `a` overlap one of `b`; complete/partial
            [50., 52.], // 11
            [53., 58.], // 12: one of `a` overlaps two of `b` completely
            [60., 61.], // 13: one of `b` overlaps two of `a` completely
            [62., 63.], // 13
            [65., 66.], // 14: one of `a` with no overlap in `b`
        ]);
        let b = CurveFaceIntersectionList::from_intervals([
            [0., 1.],   // 1
            [3., 4.],   // 2
            [6., 9.],   // 3
            [10., 12.], // 4
            [13., 15.], // 5
            [17., 19.], // 6
            [20., 22.], // 6
            [24., 27.], // 7
            [30., 32.], // 8
            [33., 34.], // 8
            [37., 41.], // 9
            [42., 43.], // 10
            [44., 46.], // 10
            [47., 51.], // 11
            [54., 55.], // 12
            [56., 57.], // 12
            [59., 64.], // 13
        ]);

        let merged = a.merge(&b);

        let expected = CurveFaceIntersectionList::from_intervals([
            [0., 1.],   // 1
            [3., 4.],   // 2
            [7., 8.],   // 3
            [10., 11.], // 4
            [14., 15.], // 5
            [18., 19.], // 6
            [20., 21.], // 6
            [24., 25.], // 7
            [26., 27.], // 7
            [31., 32.], // 8
            [33., 34.], // 8
            [37., 38.], // 9
            [39., 40.], // 9
            [42., 43.], // 10
            [44., 45.], // 10
            [48., 49.], // 11
            [50., 51.], // 11
            [54., 55.], // 12
            [56., 57.], // 12
            [60., 61.], // 13
            [62., 63.], // 13
        ]);
        assert_eq!(merged, expected);
    }
}
