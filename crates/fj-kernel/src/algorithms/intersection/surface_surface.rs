use fj_math::{Line, Point, Scalar, Vector};

use crate::objects::{CurveKind, Surface};

/// The intersection between two surfaces
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceSurfaceIntersection {
    /// The intersection curves, in the coordinates of the input surfaces
    pub local_intersection_curves: [CurveKind<2>; 2],

    /// The intersection curve, in global coordinates
    pub global_intersection_curve: CurveKind<3>,
}

impl SurfaceSurfaceIntersection {
    /// Compute the intersection between two surfaces
    pub fn compute([a, b]: [&Surface; 2]) -> Option<Self> {
        // Algorithm from Real-Time Collision Detection by Christer Ericson. See
        // section 5.4.4, Intersection of Two Planes.
        //
        // Adaptations were made to get the intersection curves in local
        // coordinates for each surface.

        let a_parametric = PlaneParametric::extract_from_surface(a);
        let b_parametric = PlaneParametric::extract_from_surface(b);

        let a = PlaneConstantNormal::from_parametric_plane(&a_parametric);
        let b = PlaneConstantNormal::from_parametric_plane(&b_parametric);

        let direction = a.normal.cross(&b.normal);

        let denom = direction.dot(&direction);
        if denom == Scalar::ZERO {
            // Comparing `denom` against zero looks fishy. It's probably better
            // to compare it against an epsilon value, but I don't know how
            // large that epsilon should be.
            //
            // I'll just leave it like that, until we had the opportunity to
            // collect some experience with this code.
            // - @hannobraun
            return None;
        }

        let origin = (b.normal * a.distance - a.normal * b.distance)
            .cross(&direction)
            / denom;
        let origin = Point { coords: origin };

        let line = Line { origin, direction };

        let curve_a = project_line_into_plane(&line, &a_parametric);
        let curve_b = project_line_into_plane(&line, &b_parametric);
        let curve_global = CurveKind::Line(Line { origin, direction });

        Some(Self {
            local_intersection_curves: [curve_a, curve_b],
            global_intersection_curve: curve_global,
        })
    }
}

/// A plane in parametric form
#[derive(Clone, Copy)]
struct PlaneParametric {
    pub origin: Point<3>,
    pub u: Vector<3>,
    pub v: Vector<3>,
}

impl PlaneParametric {
    pub fn extract_from_surface(surface: &Surface) -> Self {
        let Surface::SweptCurve(surface) = surface;
        let line = match surface.curve {
            CurveKind::Line(line) => line,
            _ => todo!("Only plane-plane intersection is currently supported."),
        };

        Self {
            origin: line.origin,
            u: line.direction,
            v: surface.path,
        }
    }
}

/// A plane in constant-normal form
struct PlaneConstantNormal {
    pub distance: Scalar,
    pub normal: Vector<3>,
}

impl PlaneConstantNormal {
    /// Extract a plane in constant-normal form from a `Surface`
    ///
    /// Panics, if the given `Surface` is not a plane.
    pub fn from_parametric_plane(plane: &PlaneParametric) -> Self {
        // Convert plane from parametric form to three-point form.
        let a = plane.origin;
        let b = plane.origin + plane.u;
        let c = plane.origin + plane.v;

        // Convert plane from three-point form to constant-normal form. See
        // Real-Time Collision Detection by Christer Ericson, section 3.6, Planes
        // and Halfspaces.
        let normal = (b - a).cross(&(c - a)).normalize();
        let distance = normal.dot(&a.coords);

        PlaneConstantNormal { distance, normal }
    }
}

fn project_line_into_plane(
    line: &Line<3>,
    plane: &PlaneParametric,
) -> CurveKind<2> {
    let line_origin_relative_to_plane = line.origin - plane.origin;
    let line_origin_in_plane = Vector::from([
        plane
            .u
            .scalar_projection_onto(&line_origin_relative_to_plane),
        plane
            .v
            .scalar_projection_onto(&line_origin_relative_to_plane),
    ]);

    let line_direction_in_plane = Vector::from([
        plane.u.scalar_projection_onto(&line.direction),
        plane.v.scalar_projection_onto(&line.direction),
    ]);

    let line = Line {
        origin: Point {
            coords: line_origin_in_plane,
        },
        direction: line_direction_in_plane,
    };

    CurveKind::Line(line)
}

#[cfg(test)]
mod tests {
    use fj_math::Transform;

    use crate::{
        algorithms::TransformObject,
        objects::{Curve, Surface},
    };

    use super::SurfaceSurfaceIntersection;

    #[test]
    fn plane_plane() {
        let xy = Surface::xy_plane();
        let xz = Surface::xz_plane();

        // Coincident and parallel planes don't have an intersection curve.
        assert_eq!(SurfaceSurfaceIntersection::compute([&xy, &xy]), None);
        assert_eq!(
            SurfaceSurfaceIntersection::compute([
                &xy,
                &xy.transform(&Transform::translation([0., 0., 1.]))
            ]),
            None,
        );

        let expected_xy = Curve::build(xy).u_axis();
        let expected_xz = Curve::build(xz).u_axis();

        assert_eq!(
            SurfaceSurfaceIntersection::compute([&xy, &xz]),
            Some(SurfaceSurfaceIntersection {
                local_intersection_curves: [
                    *expected_xy.kind(),
                    *expected_xz.kind()
                ],
                global_intersection_curve: *expected_xy.global().kind(),
            })
        );
    }
}
