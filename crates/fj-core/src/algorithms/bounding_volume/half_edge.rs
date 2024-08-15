use fj_math::{Aabb, Vector};

use crate::{
    geometry::{Geometry, Path},
    storage::Handle,
    topology::{HalfEdge, Surface, Vertex},
};

impl super::BoundingVolume<2>
    for (&Handle<HalfEdge>, &Handle<Vertex>, &Handle<Surface>)
{
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<2>> {
        let (half_edge, end_vertex, surface) = self;

        let path = geometry
            .of_curve(half_edge.curve())
            .unwrap()
            .local_on(surface)
            .unwrap()
            .path;

        match path {
            Path::Circle(circle) => {
                // Just calculate the AABB of the whole circle. This is not the
                // most precise, but it should do for now.

                let center_to_min_max =
                    Vector::from([circle.radius(), circle.radius()]);

                Some(Aabb {
                    min: circle.center() - center_to_min_max,
                    max: circle.center() + center_to_min_max,
                })
            }
            Path::Line(_) => {
                let points =
                    [half_edge.start_vertex(), end_vertex].map(|vertex| {
                        let point_curve = geometry
                            .of_vertex(vertex)
                            .unwrap()
                            .local_on(half_edge.curve())
                            .unwrap()
                            .position;

                        path.point_from_path_coords(point_curve)
                    });

                Some(Aabb::<2>::from_points(points))
            }
        }
    }
}
