use fj_math::Point;

use crate::{
    algorithms::cast_ray::{CastRay, HorizontalRayToTheRight, RaySegmentHit},
    objects::Face,
};

use super::Contains;

impl Contains<Point<2>> for Face {
    fn contains(&self, point: &Point<2>) -> bool {
        let ray = HorizontalRayToTheRight { origin: *point };

        let mut num_hits = 0;

        for cycle in self.all_cycles() {
            // We need to properly detect the ray passing the boundary at the
            // "seam" of the polygon, i.e. the vertex between the last and the
            // first segment. The logic in the loop properly takes care of that,
            // as long as we initialize the `previous_hit` variable with the
            // result of the last segment.
            let mut previous_hit = cycle
                .edges()
                .last()
                .copied()
                .and_then(|edge| edge.cast_ray(ray));

            for edge in cycle.edges() {
                let hit = edge.cast_ray(ray);

                let count_hit = match (hit, previous_hit) {
                    (Some(RaySegmentHit::Segment), _) => {
                        // We're hitting a segment right-on. Clear case.
                        true
                    }
                    (
                        Some(RaySegmentHit::UpperVertex),
                        Some(RaySegmentHit::LowerVertex),
                    )
                    | (
                        Some(RaySegmentHit::LowerVertex),
                        Some(RaySegmentHit::UpperVertex),
                    ) => {
                        // If we're hitting a vertex, only count it if we've hit
                        // the other kind of vertex right before.
                        //
                        // That means, we're passing through the polygon
                        // boundary at where two edges touch. Depending on the
                        // order in which edges are checked, we're seeing this
                        // as a hit to one edge's lower/upper vertex, then the
                        // other edge's opposite vertex.
                        //
                        // If we're seeing two of the same vertices in a row,
                        // we're not actually passing through the polygon
                        // boundary. Then we're just touching a vertex without
                        // passing through anything.
                        true
                    }
                    (Some(RaySegmentHit::Parallel), _) => {
                        // A parallel edge must be completely ignored. Its
                        // presence won't change anything, so we can treat it as
                        // if it wasn't there, and its neighbors were connected
                        // to each other.
                        continue;
                    }
                    _ => {
                        // Any other case is not a valid hit.
                        false
                    }
                };

                if count_hit {
                    num_hits += 1;
                }

                previous_hit = hit;
            }
        }

        num_hits % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        algorithms::Contains,
        objects::{Face, Surface},
    };

    #[test]
    fn ray_hits_vertex_while_passing_outside() {
        let face = Face::build(Surface::xy_plane()).polygon_from_points([
            [0., 0.],
            [2., 1.],
            [0., 2.],
        ]);

        assert_contains_point(face, [1., 1.]);
    }

    #[test]
    fn ray_hits_vertex_at_cycle_seam() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[4., 2.], [0., 4.], [0., 0.]])
            .with_hole([[1., 1.], [2., 1.], [1., 3.]]);

        assert_contains_point(face, [1., 2.]);
    }

    #[test]
    fn ray_hits_vertex_while_staying_inside() {
        let face = Face::build(Surface::xy_plane()).polygon_from_points([
            [0., 0.],
            [2., 1.],
            [3., 0.],
            [3., 4.],
        ]);

        assert_contains_point(face, [1., 1.]);
    }

    #[test]
    fn ray_hits_parallel_edge() {
        // Ray passes face boundary at a vertex.
        let face = Face::build(Surface::xy_plane()).polygon_from_points([
            [0., 0.],
            [2., 1.],
            [3., 1.],
            [0., 2.],
        ]);
        assert_contains_point(face, [1., 1.]);

        // Ray hits a vertex, but doesn't pass face boundary there.
        let face = Face::build(Surface::xy_plane()).polygon_from_points([
            [0., 0.],
            [2., 1.],
            [3., 1.],
            [4., 0.],
            [4., 5.],
        ]);
        assert_contains_point(face, [1., 1.]);
    }

    fn assert_contains_point(
        face: impl Into<Face>,
        point: impl Into<Point<2>>,
    ) {
        let face = face.into();
        let point = point.into();

        assert!(face.contains(&point));
    }
}
