use fj_interop::ext::ArrayExt;
use fj_math::{Line, Plane, Point, Scalar};

use crate::{
    objects::{Curve, GlobalCurve, Objects, Surface},
    path::{GlobalPath, SurfacePath},
    storage::Handle,
    validate::ValidationError,
};

/// The intersection between two surfaces
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceSurfaceIntersection {
    /// The intersection curves
    pub intersection_curves: [Handle<Curve>; 2],
}

impl SurfaceSurfaceIntersection {
    /// Compute the intersection between two surfaces
    pub fn compute(
        surfaces: [Handle<Surface>; 2],
        objects: &Objects,
    ) -> Result<Option<Self>, ValidationError> {
        // Algorithm from Real-Time Collision Detection by Christer Ericson. See
        // section 5.4.4, Intersection of Two Planes.
        //
        // Adaptations were made to get the intersection curves in local
        // coordinates for each surface.

        let surfaces_and_planes = surfaces.map(|surface| {
            let plane = plane_from_surface(&surface);
            (surface, plane)
        });
        let [a, b] = surfaces_and_planes.clone().map(|(_, plane)| plane);

        let (a_distance, a_normal) = a.constant_normal_form();
        let (b_distance, b_normal) = b.constant_normal_form();

        let direction = a_normal.cross(&b_normal);

        let denom = direction.dot(&direction);
        if denom == Scalar::ZERO {
            // Comparing `denom` against zero looks fishy. It's probably better
            // to compare it against an epsilon value, but I don't know how
            // large that epsilon should be.
            //
            // I'll just leave it like that, until we had the opportunity to
            // collect some experience with this code.
            // - @hannobraun
            return Ok(None);
        }

        let origin = (b_normal * a_distance - a_normal * b_distance)
            .cross(&direction)
            / denom;
        let origin = Point { coords: origin };

        let line = Line::from_origin_and_direction(origin, direction);

        let curves = surfaces_and_planes.try_map_ext(|(surface, plane)| {
            let path = SurfacePath::Line(plane.project_line(&line));
            let global_form = objects.global_curves.insert(GlobalCurve)?;

            objects
                .curves
                .insert(Curve::new(surface, path, global_form))
        })?;

        Ok(Some(Self {
            intersection_curves: curves,
        }))
    }
}

fn plane_from_surface(surface: &Surface) -> Plane {
    let (line, path) = {
        let line = match surface.u() {
            GlobalPath::Line(line) => line,
            _ => todo!("Only plane-plane intersection is currently supported."),
        };

        (line, surface.v())
    };

    Plane::from_parametric(line.origin(), line.direction(), path)
}

#[cfg(test)]
mod tests {
    use fj_math::Transform;
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::transform::TransformObject, builder::CurveBuilder,
        insert::Insert, objects::Objects, partial::PartialCurve,
    };

    use super::SurfaceSurfaceIntersection;

    #[test]
    fn plane_plane() -> anyhow::Result<()> {
        let objects = Objects::new();

        let xy = objects.surfaces.xy_plane();
        let xz = objects.surfaces.xz_plane();

        // Coincident and parallel planes don't have an intersection curve.
        assert_eq!(
            SurfaceSurfaceIntersection::compute(
                [
                    xy.clone(),
                    xy.clone().transform(
                        &Transform::translation([0., 0., 1.],),
                        &objects
                    )?
                ],
                &objects
            )?,
            None,
        );

        let expected_xy = PartialCurve {
            surface: Some(xy.clone()),
            ..Default::default()
        }
        .update_as_u_axis()
        .build(&objects)?
        .insert(&objects)?;
        let expected_xz = PartialCurve {
            surface: Some(xz.clone()),
            ..Default::default()
        }
        .update_as_u_axis()
        .build(&objects)?
        .insert(&objects)?;

        assert_eq!(
            SurfaceSurfaceIntersection::compute([xy, xz], &objects)?,
            Some(SurfaceSurfaceIntersection {
                intersection_curves: [expected_xy, expected_xz],
            })
        );
        Ok(())
    }
}
