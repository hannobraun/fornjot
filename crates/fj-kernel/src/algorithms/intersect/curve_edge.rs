use fj_math::{Point, Segment};

use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, HalfEdge},
};

use super::LineSegmentIntersection;

/// The intersection between a [`Curve`] and a [`HalfEdge`]
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
    /// computed. Panics, if a different type of [`Curve`] or [`HalfEdge`] is
    /// passed.
    pub fn compute(curve: &Curve, half_edge: &HalfEdge) -> Option<Self> {
        let curve_as_line = match curve.path() {
            SurfacePath::Line(line) => line,
            _ => todo!("Curve-edge intersection only supports lines"),
        };

        let edge_as_segment = {
            let edge_curve_as_line = match half_edge.curve().path() {
                SurfacePath::Line(line) => line,
                _ => {
                    todo!("Curve-edge intersection only supports line segments")
                }
            };

            let edge_vertices = half_edge.vertices().clone().map(|vertex| {
                edge_curve_as_line.point_from_line_coords(vertex.position())
            });

            Segment::from_points(edge_vertices)
        };

        let intersection =
            LineSegmentIntersection::compute(&curve_as_line, &edge_as_segment)?;

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
        builder::{CurveBuilder, HalfEdgeBuilder},
        objects::{HalfEdge, Objects},
        partial::{HasPartial, PartialCurve},
    };

    use super::CurveEdgeIntersection;

    #[test]
    fn compute_edge_in_front_of_curve_origin() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let mut curve = PartialCurve {
            surface: Some(surface.clone()),
            ..Default::default()
        };
        curve.update_as_u_axis();
        let curve = curve.build(&mut objects)?;
        let half_edge = HalfEdge::partial()
            .update_as_line_segment_from_points(surface, [[1., -1.], [1., 1.]])
            .build(&mut objects)?;

        let intersection = CurveEdgeIntersection::compute(&curve, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([1.])
            })
        );
        Ok(())
    }

    #[test]
    fn compute_edge_behind_curve_origin() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let mut curve = PartialCurve {
            surface: Some(surface.clone()),
            ..Default::default()
        };
        curve.update_as_u_axis();
        let curve = curve.build(&mut objects)?;
        let half_edge = HalfEdge::partial()
            .update_as_line_segment_from_points(
                surface,
                [[-1., -1.], [-1., 1.]],
            )
            .build(&mut objects)?;

        let intersection = CurveEdgeIntersection::compute(&curve, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([-1.])
            })
        );
        Ok(())
    }

    #[test]
    fn compute_edge_parallel_to_curve() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let mut curve = PartialCurve {
            surface: Some(surface.clone()),
            ..Default::default()
        };
        curve.update_as_u_axis();
        let curve = curve.build(&mut objects)?;
        let half_edge = HalfEdge::partial()
            .update_as_line_segment_from_points(
                surface,
                [[-1., -1.], [1., -1.]],
            )
            .build(&mut objects)?;

        let intersection = CurveEdgeIntersection::compute(&curve, &half_edge);

        assert!(intersection.is_none());
        Ok(())
    }

    #[test]
    fn compute_edge_on_curve() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let mut curve = PartialCurve {
            surface: Some(surface.clone()),
            ..Default::default()
        };
        curve.update_as_u_axis();
        let curve = curve.build(&mut objects)?;
        let half_edge = HalfEdge::partial()
            .update_as_line_segment_from_points(surface, [[-1., 0.], [1., 0.]])
            .build(&mut objects)?;

        let intersection = CurveEdgeIntersection::compute(&curve, &half_edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Coincident {
                points_on_curve: [Point::from([-1.]), Point::from([1.]),]
            })
        );
        Ok(())
    }
}
