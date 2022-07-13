use fj_math::{Line, Point, Scalar, Vector};

use crate::objects::{Curve, Surface};

/// Test intersection between two surfaces
pub fn surface_surface(a: &Surface, b: &Surface) -> Option<Curve<3>> {
    // Algorithm from Real-Time Collision Detection by Christer Ericson. See
    // section 5.4.4, Intersection of Two Planes.

    let a = extract_plane(a);
    let b = extract_plane(b);

    let direction = a.normal.cross(&b.normal);

    let denom = direction.dot(&direction);
    if denom == Scalar::ZERO {
        // Comparing `denom` against zero looks fishy. It's probably better to
        // compare it against an epsilon value, but I don't know how large that
        // epsilon should be.
        //
        // I'll just leave it like that, until we had the opportunity to collect
        // some experience with this code.
        // - @hannobraun
        return None;
    }

    let origin = (b.normal * a.distance - a.normal * b.distance)
        .cross(&direction)
        / denom;
    let origin = Point { coords: origin };

    Some(Curve::Line(Line { origin, direction }))
}

/// A plane in constant-normal form
struct PlaneConstantNormal {
    pub distance: Scalar,
    pub normal: Vector<3>,
}

/// Extract a plane in constant-normal form from a `Surface`
///
/// Panics, if the given `Surface` is not a plane.
fn extract_plane(surface: &Surface) -> PlaneConstantNormal {
    let Surface::SweptCurve(surface) = surface;
    let line = match surface.curve {
        Curve::Line(line) => line,
        _ => todo!("Only plane-plane intersection is currently supported."),
    };

    // Convert plane from parametric form to three-point form.
    let a = line.origin;
    let b = line.origin + line.direction;
    let c = line.origin + surface.path;

    // Convert plane from three-point form to constant-normal form. See
    // Real-Time Collision Detection by Christer Ericson, section 3.6, Planes
    // and Halfspaces.
    let normal = (b - a).cross(&(c - a)).normalize();
    let distance = normal.dot(&a.coords);

    PlaneConstantNormal { distance, normal }
}

#[cfg(test)]
mod tests {
    use fj_math::Transform;

    use crate::objects::{Curve, Surface};

    use super::surface_surface;

    #[test]
    fn plane_plane() {
        let xy = Surface::xy_plane();
        let xz = Surface::xz_plane();

        assert_eq!(surface_surface(&xy, &xy), None);
        assert_eq!(
            surface_surface(
                &xy,
                &xy.transform(&Transform::translation([0., 0., 1.]))
            ),
            None,
        );
        assert_eq!(surface_surface(&xy, &xz), Some(Curve::x_axis()));
    }
}
