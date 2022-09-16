//! Intersection between a ray and a face, in 3D

use fj_math::{Point, Scalar, Vector};

use crate::{
    algorithms::intersect::face_point::FacePointIntersection,
    objects::{Face, HalfEdge, Vertex},
    path::GlobalPath,
};

use super::{HorizontalRayToTheRight, Intersect};

impl Intersect for (&HorizontalRayToTheRight<3>, &Face) {
    type Intersection = RayFaceIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, face) = self;

        let (plane_origin, plane_direction_1, plane_direction_2) =
            match face.surface().u() {
                GlobalPath::Circle(_) => todo!(
                    "Casting a ray against a swept circle is not supported yet"
                ),
                GlobalPath::Line(line) => {
                    (line.origin(), line.direction(), face.surface().v())
                }
            };

        let plane_and_ray_are_parallel = {
            let plane_normal = plane_direction_1.cross(&plane_direction_2);
            let ray_direction = Vector::from([1., 0., 0.]);

            plane_normal.dot(&ray_direction) == Scalar::ZERO
        };

        if plane_and_ray_are_parallel {
            let a = plane_origin;
            let b = plane_origin + plane_direction_1;
            let c = plane_origin + plane_direction_2;
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
            plane_direction_1.y * plane_direction_2.z,
            plane_direction_1.z * plane_direction_2.y,
            "Plane and ray are parallel; should have been ruled out previously"
        );

        // Let's figure out the intersection between the ray and the plane.
        let (t, u, v) = {
            // The following math would get *very* unwieldy with those
            // full-length variable names. Let's define some short-hands.
            let orx = ray.origin.x;
            let ory = ray.origin.y;
            let orz = ray.origin.z;
            let opx = plane_origin.x;
            let opy = plane_origin.y;
            let opz = plane_origin.z;
            let d1x = plane_direction_1.x;
            let d1y = plane_direction_1.y;
            let d1z = plane_direction_1.z;
            let d2x = plane_direction_2.x;
            let d2y = plane_direction_2.y;
            let d2z = plane_direction_2.z;

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
    RayHitsEdge(HalfEdge),

    /// The ray hits a vertex
    RayHitsVertex(Vertex),
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
        objects::{Face, Surface},
        stores::Stores,
    };

    #[test]
    fn ray_misses_whole_surface() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::yz_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([-1., 0., 0.], &stores);

        assert_eq!((&ray, &face).intersect(), None);
    }

    #[test]
    fn ray_hits_face() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::yz_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([1., 0., 0.], &stores);

        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsFace)
        );
    }

    #[test]
    fn ray_hits_surface_but_misses_face() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::yz_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([0., 0., 2.], &stores);

        assert_eq!((&ray, &face).intersect(), None);
    }

    #[test]
    fn ray_hits_edge() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::yz_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([1., 1., 0.], &stores);

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
    }

    #[test]
    fn ray_hits_vertex() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::yz_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([1., 1., 1.], &stores);

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
    }

    #[test]
    fn ray_is_parallel_to_surface_and_hits() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::xy_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face();

        assert_eq!(
            (&ray, &face).intersect(),
            Some(RayFaceIntersection::RayHitsFaceAndAreParallel)
        )
    }

    #[test]
    fn ray_is_parallel_to_surface_and_misses() {
        let stores = Stores::new();

        let ray = HorizontalRayToTheRight::from([0., 0., 0.]);

        let face = Face::build(&stores, Surface::xy_plane())
            .polygon_from_points([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
            .into_face()
            .translate([0., 0., 1.], &stores);

        assert_eq!((&ray, &face).intersect(), None)
    }
}
