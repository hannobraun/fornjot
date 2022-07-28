use fj_math::{Point, Segment};
use parry2d_f64::query::{Ray, RayCast};

use crate::objects::{Curve, Edge};

/// The intersection between a [`Curve`] and an [`Edge`], in curve coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CurveEdgeIntersection {
    /// The curve and edge intersect at a point
    Point {
        /// The intersection point, in curve coordinates on the curve
        point_on_curve: Point<1>,
    },

    /// The edge lies on the curve
    Coincident {
        /// The first vertex of the edge, in curve coordinates
        a_on_curve: Point<1>,

        /// The second vertex of the edge, in curve coordinates
        b_on_curve: Point<1>,
    },
}

impl CurveEdgeIntersection {
    /// Compute the intersection
    ///
    /// # Panics
    ///
    /// Currently, only intersections between lines and line segments can be
    /// computed. Panics, if a different type of [`Curve`] or [`Edge`] is
    /// passed.
    pub fn compute(curve: &Curve<2>, edge: &Edge) -> Option<Self> {
        let curve_as_line = match curve {
            Curve::Line(line) => line,
            _ => todo!("Curve-edge intersection only supports lines"),
        };

        let edge_curve_as_line = match edge.curve().local_form() {
            Curve::Line(line) => line,
            _ => {
                todo!("Curve-edge intersection only supports line segments")
            }
        };

        let edge_vertices = match edge.vertices().get() {
            Some(vertices) => vertices.map(|vertex| {
                edge_curve_as_line.point_from_line_coords(vertex.position())
            }),
            None => todo!(
                "Curve-edge intersection does not support continuous edges"
            ),
        };

        let edge_as_segment = Segment::from_points(edge_vertices);

        if curve_as_line.is_coincident_with(edge_curve_as_line) {
            let [a_on_curve, b_on_curve] = edge_vertices
                .map(|vertex| curve_as_line.point_to_line_coords(vertex));

            return Some(Self::Coincident {
                a_on_curve,
                b_on_curve,
            });
        }

        let ray = Ray {
            origin: curve_as_line.origin.to_na(),
            dir: curve_as_line.direction.to_na(),
        };
        let ray_inv = Ray {
            origin: curve_as_line.origin.to_na(),
            dir: -curve_as_line.direction.to_na(),
        };

        let result = edge_as_segment.to_parry().cast_local_ray(
            &ray,
            f64::INFINITY,
            false,
        );
        let result_inv = edge_as_segment.to_parry().cast_local_ray(
            &ray_inv,
            f64::INFINITY,
            false,
        );

        if let Some(result) = result {
            return Some(Self::Point {
                point_on_curve: Point::from([result]),
            });
        }
        if let Some(result_inv) = result_inv {
            return Some(Self::Point {
                point_on_curve: Point::from([-result_inv]),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::objects::{Curve, Edge, Surface};

    use super::CurveEdgeIntersection;

    #[test]
    fn compute_edge_in_front_of_curve_origin() {
        let surface = Surface::xy_plane();
        let curve = Curve::u_axis();
        let edge = Edge::build()
            .line_segment_from_points(&surface, [[1., -1.], [1., 1.]]);

        let intersection = CurveEdgeIntersection::compute(&curve, &edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([1.])
            })
        );
    }

    #[test]
    fn compute_edge_behind_curve_origin() {
        let surface = Surface::xy_plane();
        let curve = Curve::u_axis();
        let edge = Edge::build()
            .line_segment_from_points(&surface, [[-1., -1.], [-1., 1.]]);

        let intersection = CurveEdgeIntersection::compute(&curve, &edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Point {
                point_on_curve: Point::from([-1.])
            })
        );
    }

    #[test]
    fn compute_edge_parallel_to_curve() {
        let surface = Surface::xy_plane();
        let curve = Curve::u_axis();
        let edge = Edge::build()
            .line_segment_from_points(&surface, [[-1., -1.], [1., -1.]]);

        let intersection = CurveEdgeIntersection::compute(&curve, &edge);

        assert!(intersection.is_none());
    }

    #[test]
    fn compute_edge_on_curve() {
        let surface = Surface::xy_plane();
        let curve = Curve::u_axis();
        let edge = Edge::build()
            .line_segment_from_points(&surface, [[-1., 0.], [1., 0.]]);

        let intersection = CurveEdgeIntersection::compute(&curve, &edge);

        assert_eq!(
            intersection,
            Some(CurveEdgeIntersection::Coincident {
                a_on_curve: Point::from([-1.]),
                b_on_curve: Point::from([1.]),
            })
        );
    }
}
