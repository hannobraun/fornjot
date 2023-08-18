use fj_math::{Point, Segment};

use crate::{geometry::SurfacePath, objects::Edge};

use super::LineSegmentIntersection;

/// The intersection between a curve and an [`Edge`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CurveEdgeIntersection {
    /// The curve and edge intersect at a point
    Point {
        /// The intersection point, in curve coordinates on the curve
        point_on_curve: Point<1>,
    },

    /// The edge lies on the curve
    Coincident {
        /// The end points of the edge, in curve coordinates on the curve
        points_on_curve: [Point<1>; 2],
    },
}

impl CurveEdgeIntersection {
    /// Compute the intersection
    ///
    /// # Panics
    ///
    /// Currently, only intersections between lines and line segments can be
    /// computed. Panics, if a different type of curve or [`Edge`] is passed.
    pub fn compute(path: &SurfacePath, edge: &Edge) -> Option<Self> {
        let path_as_line = match path {
            SurfacePath::Line(line) => line,
            _ => todo!("Curve-edge intersection only supports lines"),
        };

        let edge_as_segment = {
            let edge_path_as_line = match edge.path() {
                SurfacePath::Line(line) => line,
                _ => {
                    todo!("Curve-edge intersection only supports line segments")
                }
            };

            let edge_vertices = edge
                .boundary()
                .inner
                .map(|point| edge_path_as_line.point_from_line_coords(point));

            Segment::from_points(edge_vertices)
        };

        let intersection =
            LineSegmentIntersection::compute(path_as_line, &edge_as_segment)?;

        let intersection = match intersection {
            LineSegmentIntersection::Point { point_on_line } => Self::Point {
                point_on_curve: point_on_line,
            },
            LineSegmentIntersection::Coincident { points_on_line } => {
                Self::Coincident {
                    points_on_curve: points_on_line,
                }
            }
        };

        Some(intersection)
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        geometry::SurfacePath, objects::Edge, operations::BuildEdge,
        services::Services,
    };

    use super::CurveEdgeIntersection;

    #[test]
    fn compute_edge_in_front_of_curve_origin() {
        let mut services = Services::new();

        let path = SurfacePath::u_axis();
        let half_edge =
            Edge::line_segment([[1., -1.], [1., 1.]], None, &mut services);

        let intersection = CurveEdgeIntersection::compute(&path, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([1.])
            })
        );
    }

    #[test]
    fn compute_edge_behind_curve_origin() {
        let mut services = Services::new();

        let path = SurfacePath::u_axis();
        let half_edge =
            Edge::line_segment([[-1., -1.], [-1., 1.]], None, &mut services);

        let intersection = CurveEdgeIntersection::compute(&path, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([-1.])
            })
        );
    }

    #[test]
    fn compute_edge_parallel_to_curve() {
        let mut services = Services::new();

        let path = SurfacePath::u_axis();
        let half_edge =
            Edge::line_segment([[-1., -1.], [1., -1.]], None, &mut services);

        let intersection = CurveEdgeIntersection::compute(&path, &half_edge);

        assert!(intersection.is_none());
    }

    #[test]
    fn compute_edge_on_curve() {
        let mut services = Services::new();

        let path = SurfacePath::u_axis();
        let half_edge =
            Edge::line_segment([[-1., 0.], [1., 0.]], None, &mut services);

        let intersection = CurveEdgeIntersection::compute(&path, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Coincident {
                points_on_curve: [Point::from([-1.]), Point::from([1.]),]
            })
        );
    }
}
