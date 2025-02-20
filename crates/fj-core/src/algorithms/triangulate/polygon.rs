use fj_interop::ext::SliceExt;
use fj_math::{LineSegment, Point, PolyChain, Triangle};

use crate::algorithms::intersect::{
    HorizontalRayToTheRight, Intersect, ray_segment::RaySegmentIntersection,
};

#[derive(Default)]
pub struct Polygon {
    exterior: PolyChain<2>,
    interiors: Vec<PolyChain<2>>,
}

impl Polygon {
    /// Construct an instance of `Polygon`
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_exterior(mut self, exterior: impl Into<PolyChain<2>>) -> Self {
        self.exterior = exterior.into();
        self
    }

    pub fn with_interiors(
        mut self,
        interiors: impl IntoIterator<Item = impl Into<PolyChain<2>>>,
    ) -> Self {
        self.interiors.extend(interiors.into_iter().map(Into::into));
        self
    }

    #[cfg(test)]
    pub fn invert_winding(mut self) -> Self {
        self.exterior = self.exterior.reverse();

        for interior in &mut self.interiors {
            *interior = interior.clone().reverse();
        }

        self
    }

    pub fn contains_triangle(&self, triangle: impl Into<Triangle<2>>) -> bool {
        let [a, b, c] = triangle.into().points;

        let mut might_be_hole = true;

        for &edge in [a, b, c, a].as_slice().array_windows_ext() {
            let edge = LineSegment::from(edge);

            let is_exterior_edge = self.contains_exterior_edge(edge);
            let is_interior_edge = self.contains_interior_edge(edge);

            // If the triangle edge is not an interior edge of the polygon, we
            // can rule out that the triangle is identical with a hole in the
            // polygon.
            if !is_interior_edge {
                might_be_hole = false;
            }

            // If the triangle edge is an edge of the face, we don't need to
            // take a closer look.
            if is_exterior_edge || is_interior_edge {
                continue;
            }

            // To determine if the edge is within the polygon, we determine if
            // its center point is in the polygon.
            //
            // Since we already checked above, whether the triangle edge is a
            // polygon edge (and if we reached this point, it isn't), we don't
            // need to care about the distinction between "inside the polygon"
            // and "on the polygon boundary".
            if !self.contains_point(edge.center()) {
                // The segment is outside of the face. This means we can throw
                // away the whole triangle.
                return false;
            }
        }

        // We haven't ruled out that the triangle is a polygon hole. Since we
        // checked all its edges, this means we now know for certain that is is.
        if might_be_hole {
            return false;
        }

        // If we didn't throw away the triangle up till now, this means all its
        // edges are within the face.
        true
    }

    fn contains_exterior_edge(&self, edge: LineSegment<2>) -> bool {
        self.exterior.segments().contains(&edge)
            || self.exterior.segments().contains(&edge.reverse())
    }

    fn contains_interior_edge(&self, edge: LineSegment<2>) -> bool {
        let mut contains = false;

        for chain in &self.interiors {
            contains |= chain.segments().contains(&edge);
            contains |= chain.segments().contains(&edge.reverse());
        }

        contains
    }

    /// Check whether the polygon contains a point
    ///
    /// # Implementation Note
    ///
    /// This code is being duplicated by the `Contains<Point<2>>` implementation
    /// for `Face`. It would be nice to be able to consolidate the duplication,
    /// but this has turned out to be difficult.
    fn contains_point(&self, point: impl Into<Point<2>>) -> bool {
        let ray = HorizontalRayToTheRight {
            origin: point.into(),
        };

        let mut num_hits = 0;

        for chain in Some(&self.exterior).into_iter().chain(&self.interiors) {
            let edges = chain.segments();

            // We need to properly detect the ray passing the boundary at the
            // "seam" of the polygon, i.e. the vertex between the last and the
            // first segment. The logic in the loop properly takes care of that,
            // as long as we initialize the `previous_hit` variable with the
            // result of the last segment.
            let mut previous_hit = edges
                .last()
                .copied()
                .and_then(|edge| (&ray, &edge).intersect());

            for edge in edges {
                let hit = (&ray, &edge).intersect();

                let count_hit = match (hit, previous_hit) {
                    (
                        Some(
                            RaySegmentIntersection::RayStartsOnSegment
                            | RaySegmentIntersection::RayStartsOnOnFirstVertex
                            | RaySegmentIntersection::RayStartsOnSecondVertex,
                        ),
                        _,
                    ) => {
                        // If the ray starts on the boundary of the polygon,
                        // there's nothing else to check. By the definition of
                        // this intersection test, the polygon contains the
                        // point.
                        return true;
                    }
                    (Some(RaySegmentIntersection::RayHitsSegment), _) => {
                        // We're hitting a segment right-on. Clear case.
                        true
                    }
                    (
                        Some(RaySegmentIntersection::RayHitsUpperVertex),
                        Some(RaySegmentIntersection::RayHitsLowerVertex),
                    )
                    | (
                        Some(RaySegmentIntersection::RayHitsLowerVertex),
                        Some(RaySegmentIntersection::RayHitsUpperVertex),
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
                    (Some(RaySegmentIntersection::RayHitsSegmentAndAreParallel), _) => {
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
    use fj_math::{Point, PolyChain};

    use super::Polygon;

    #[test]
    fn contains_triangle_with_triangular_hole() {
        let a = [0., 0.];
        let b = [3., 0.];
        let c = [0., 3.];

        let d = [1., 1.];
        let e = [2., 1.];
        let f = [1., 2.];

        let polygon = Polygon::new()
            .with_exterior(PolyChain::from([a, b, c]).close())
            .with_interiors([PolyChain::from([d, e, f]).close()]);

        assert!(!polygon.contains_triangle([d, e, f]));
    }

    #[test]
    fn contains_point_ray_hits_vertex_while_passing_outside() {
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [0., 2.];

        let polygon =
            Polygon::new().with_exterior(PolyChain::from([a, b, c]).close());

        assert_contains_point(polygon, [1., 1.]);
    }

    #[test]
    fn contains_point_ray_hits_vertex_at_polygon_seam() {
        let a = [4., 2.];
        let b = [0., 4.];
        let c = [0., 0.];

        let d = [1., 1.];
        let e = [2., 1.];
        let f = [1., 3.];

        let polygon = Polygon::new()
            .with_exterior(PolyChain::from([a, b, c]).close())
            .with_interiors([PolyChain::from([d, e, f]).close()]);

        assert_contains_point(polygon, [1., 2.]);
    }

    #[test]
    fn contains_point_ray_hits_vertex_while_staying_inside() {
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [3., 0.];
        let d = [3., 4.];

        let polygon =
            Polygon::new().with_exterior(PolyChain::from([a, b, c, d]).close());

        assert_contains_point(polygon, [1., 1.]);
    }

    #[test]
    fn contains_ray_hits_parallel_edge() {
        // Ray passes polygon boundary at a vertex.
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [3., 1.];
        let d = [0., 2.];
        let polygon =
            Polygon::new().with_exterior(PolyChain::from([a, b, c, d]).close());
        assert_contains_point(polygon, [1., 1.]);

        // Ray hits a vertex, but doesn't pass polygon boundary there.
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [3., 1.];
        let d = [4., 0.];
        let e = [4., 5.];
        let polygon = Polygon::new()
            .with_exterior(PolyChain::from([a, b, c, d, e]).close());
        assert_contains_point(polygon, [1., 1.]);
    }

    fn assert_contains_point(polygon: Polygon, point: impl Into<Point<2>>) {
        let point = point.into();

        assert!(polygon.contains_point(point));
        assert!(polygon.invert_winding().contains_point(point,));
    }
}
