//! Intersection between a ray and a face, in 3D

use fj_math::{Plane, Point, Scalar};

use crate::{
    algorithms::intersect::face_point::FacePointIntersection,
    objects::{Face, HalfEdge, Vertex},
    path::GlobalPath,
    storage::Handle,
};

use super::{HorizontalRayToTheRight, Intersect};

impl Intersect for (&HorizontalRayToTheRight<3>, &Handle<Face>) {
    type Intersection = RayFaceIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, face) = self;

        let plane = match face.surface().u() {
            GlobalPath::Circle(_) => todo!(
                "Casting a ray against a swept circle is not supported yet"
            ),
            GlobalPath::Line(line) => Plane::from_parametric(
                line.origin(),
                line.direction(),
                face.surface().v(),
            ),
        };

        if plane.is_parallel_to_vector(&ray.direction()) {
            let a = plane.origin();
            let b = plane.origin() + plane.u();
            let c = plane.origin() + plane.v();
            let d = ray.origin;

            let [a, b, c, d] = [a, b, c, d]
                .map(|point| [point.x, point.y, point.z])
                .map(|point| point.map(Scalar::into_f64));

            if robust_predicates::orient3d(&a, &b, &c, &d) == 0. {
                return Some(RayFaceIntersection::RayHitsFaceAndAreParallel);
            } else {
                return None;
            }
        }

        // The pattern in this assertion resembles `ax*by = ay*bx`, which holds
        // true if the vectors `a = (ax, ay)` and `b = (bx, by)` are parallel.
        //
        // We're looking at the plane's direction vectors here, but we're
        // ignoring their x-components. By doing that, we're essentially
        // projecting those vectors into the yz-plane.
        //
        // This means that the following assertion verifies that the projections
        // of the plane's direction vectors into the yz-plane are not parallel.
        // If they were, then the plane could only be parallel to the x-axis,
        // and thus our ray.
        //
        // We already handled the case of the ray and plane being parallel
        // above. The following assertion should thus never be triggered.
        assert_ne!(
            plane.u().y * plane.v().z,
            plane.u().z * plane.v().y,
            "Plane and ray are parallel; should have been ruled out previously"
        );

        // Let's figure out the intersection between the ray and the plane.
        let (t, u, v) = {
            // The following math would get *very* unwieldy with those
            // full-length variable names. Let's define some short-hands.
            let orx = ray.origin.x;
            let ory = ray.origin.y;
            let orz = ray.origin.z;
            let opx = plane.origin().x;
            let opy = plane.origin().y;
            let opz = plane.origin().z;
            let d1x = plane.u().x;
            let d1y = plane.u().y;
            let d1z = plane.u().z;
            let d2x = plane.v().x;
            let d2y = plane.v().y;
            let d2z = plane.v().z;

            // Let's figure out where the intersection between the ray and the
            // plane is. By equating the parametric equations of the ray and the
            // plane, we get a vector equation, which in turn gives us a system
            // of three equations with three unknowns: `t` (for the ray) and
            // `u`/`v` (for the plane).
            //
            // Since the ray's direction vector is `(1, 0, 0)`, it works out
            // such that `t` is not in the equations for y and z, meaning we can
            // solve those equations for `u` and `v` independently.
            //
            // By doing some math, we get the following solutions:
            let v = (d1y * (orz - opz) + (opy - ory) * d1z)
                / (d1y * d2z - d2y * d1z);
            let u = (ory - opy - d2y * v) / d1y;
            let t = opx - orx + d1x * u + d2x * v;

            (t, u, v)
        };

        if t < Scalar::ZERO {
            // Ray points away from plane.
            return None;
        }

        let point = Point::from([u, v]);
        let intersection = match (face, &point).intersect()? {
            FacePointIntersection::PointIsInsideFace => {
                RayFaceIntersection::RayHitsFace
            }
            FacePointIntersection::PointIsOnEdge(edge) => {
                RayFaceIntersection::RayHitsEdge(edge)
            }
            FacePointIntersection::PointIsOnVertex(vertex) => {
                RayFaceIntersection::RayHitsVertex(vertex)
            }
        };

        Some(intersection)
    }
}

/// A hit between a ray and a face
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum RayFaceIntersection {
    /// The ray hits the face itself
    RayHitsFace,

    /// The ray is parallel to the face
    RayHitsFaceAndAreParallel,

    /// The ray hits an edge
    RayHitsEdge(Handle<HalfEdge>),

    /// The ray hits a vertex
    RayHitsVertex(Handle<Vertex>),
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        algorithms::{
            intersect::{
                ray_face::RayFaceIntersection, HorizontalRayToTheRight,
                Intersect,
            },
            transform::TransformObject,
        },
        iter::ObjectIters,
        objects::{Face, Objects},
    };

    #[test]
    fn ray_misses_whole_surface() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.yz_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([-1., 0., 0.], &objects)?;

        assert_eq!((&ray, &face).intersect(), None);
        Ok(())
    }

    #[test]
    fn ray_hits_face() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.yz_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([1., 0., 0.], &objects)?;

        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsFace)
        );
        Ok(())
    }

    #[test]
    fn ray_hits_surface_but_misses_face() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.yz_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([0., 0., 2.], &objects)?;

        assert_eq!((&ray, &face).intersect(), None);
        Ok(())
    }

    #[test]
    fn ray_hits_edge() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.yz_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([1., 1., 0.], &objects)?;

        let edge = face
            .half_edge_iter()
            .find(|edge| {
                let [a, b] = edge.vertices();
                a.global_form().position() == Point::from([1., 0., 1.])
                    && b.global_form().position() == Point::from([1., 0., -1.])
            })
            .unwrap();
        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsEdge(edge.clone()))
        );
        Ok(())
    }

    #[test]
    fn ray_hits_vertex() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.yz_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([1., 1., 1.], &objects)?;

        let vertex = face
            .vertex_iter()
            .find(|vertex| {
                vertex.global_form().position() == Point::from([1., 0., 0.])
            })
            .unwrap();
        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsVertex(vertex.clone()))
        );
        Ok(())
    }

    #[test]
    fn ray_is_parallel_to_surface_and_hits() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.xy_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?;

        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsFaceAndAreParallel)
        );

        Ok(())
    }

    #[test]
    fn ray_is_parallel_to_surface_and_misses() -> anyhow::Result<()> {
        let objects = Objects::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let surface = objects.surfaces.xy_plane();
        let face = Face::builder()
            .with_surface(surface)
            .with_exterior_polygon_from_points([
                [-1., -1.],
                [1., -1.],
                [1., 1.],
                [-1., 1.],
            ])
            .build(&objects)?
            .translate([0., 0., 1.], &objects)?;

        assert_eq!((&ray, &face).intersect(), None);
        Ok(())
    }
}
