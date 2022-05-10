use fj_interop::debug::{DebugInfo, TriangleEdgeCheck};
use fj_math::{Point, PolyChain, Segment};

use crate::geometry::Surface;

use super::ray::{Hit, HorizontalRayToTheRight};

pub struct Polygon {
    surface: Surface,
    exterior: PolyChain<2>,
    interiors: Vec<PolyChain<2>>,
}

impl Polygon {
    /// Construct an instance of `Polygon`
    ///
    /// # Implementation note
    ///
    /// This method takes a `Surface`, but `Polygon` only uses that for
    /// generating debug info. It might be better, if `Polygon` had a field
    /// where it stored debug info specific to its algorithm. Then code using
    /// `Polygon` could access that `Polygon`-specific debug info and translate
    /// that into `DebugInfo`, as necessary.
    ///
    /// This would have the advantage of removing this dependency on `Surface`.
    /// It would also make the test code a bit cleaner, as it wouldn't have to
    /// bother with the `DebugInfo` anymore. Also, the `Polygon`-specific debug
    /// info could potentially be more useful in test code, as a debugging tool
    /// there.
    pub fn new(surface: Surface) -> Self {
        Self {
            surface,
            exterior: PolyChain::new(),
            interiors: Vec::new(),
        }
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

    pub fn contains_triangle(
        &self,
        triangle: [impl Into<Point<2>>; 3],
        debug_info: &mut DebugInfo,
    ) -> bool {
        let [a, b, c] = triangle.map(Into::into);

        let mut might_be_hole = true;

        for edge in [a, b, c, a].windows(2) {
            // This can't panic, as we passed `2` to `windows`. It can be
            // cleaned up a bit, once `array_windows` is stable.
            let edge = Segment::from([edge[0], edge[1]]);

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
            if !self.contains_point(edge.center(), debug_info) {
                // The segment is outside of the face. This means we can throw
                // away the whole triangle.
                return false;
            }
        }

        // We haven't rules out that the triangle is a polygon hole. Since we
        // checked all its edges, this means we now know for certain that is is.
        if might_be_hole {
            return false;
        }

        // If we didn't throw away the triangle up till now, this means all its
        // edges are within the face.
        true
    }

    pub fn contains_exterior_edge(&self, edge: Segment<2>) -> bool {
        self.exterior.segments().contains(&edge)
            || self.exterior.segments().contains(&edge.reverse())
    }

    pub fn contains_interior_edge(&self, edge: Segment<2>) -> bool {
        let mut contains = false;

        for chain in &self.interiors {
            contains |= chain.segments().contains(&edge);
            contains |= chain.segments().contains(&edge.reverse());
        }

        contains
    }

    pub fn contains_point(
        &self,
        point: impl Into<Point<2>>,
        debug_info: &mut DebugInfo,
    ) -> bool {
        let ray = HorizontalRayToTheRight {
            origin: point.into(),
        };

        let mut check = TriangleEdgeCheck::new(
            self.surface.convert_point_from_surface_coords(&ray.origin),
        );

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
                .and_then(|edge| ray.hits_segment(edge));

            for edge in edges {
                let hit = ray.hits_segment(edge);

                let count_hit = match (hit, previous_hit) {
                    (Some(Hit::Segment), _) => {
                        // We're hitting a segment right-on. Clear case.
                        true
                    }
                    (Some(Hit::UpperVertex), Some(Hit::LowerVertex))
                    | (Some(Hit::LowerVertex), Some(Hit::UpperVertex)) => {
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
                    (Some(Hit::Parallel), _) => {
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

                    let edge =
                        Segment::from_points(edge.points().map(|point| {
                            self.surface
                                .convert_point_from_surface_coords(&point)
                        }));
                    check.hits.push(edge);
                }

                previous_hit = hit;
            }
        }

        debug_info.triangle_edge_checks.push(check);

        num_hits % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::debug::DebugInfo;
    use fj_math::{Point, PolyChain};

    use crate::geometry::Surface;

    use super::Polygon;

    #[test]
    fn contains_triangle_with_triangular_hole() {
        let a = [0., 0.];
        let b = [3., 0.];
        let c = [0., 3.];

        let d = [1., 1.];
        let e = [2., 1.];
        let f = [1., 2.];

        let polygon = Polygon::new(Surface::xy_plane())
            .with_exterior(PolyChain::from([a, b, c]).close())
            .with_interiors([PolyChain::from([d, e, f]).close()]);

        assert!(!polygon.contains_triangle([d, e, f], &mut DebugInfo::new()));
    }

    #[test]
    fn contains_point_ray_hits_vertex_while_passing_outside() {
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [0., 2.];

        let polygon = Polygon::new(Surface::xy_plane())
            .with_exterior(PolyChain::from([a, b, c]).close());

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

        let polygon = Polygon::new(Surface::xy_plane())
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

        let polygon = Polygon::new(Surface::xy_plane())
            .with_exterior(PolyChain::from([a, b, c, d]).close());

        assert_contains_point(polygon, [1., 1.]);
    }

    #[test]
    fn contains_ray_hits_parallel_edge() {
        // Ray passes polygon boundary at a vertex.
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [3., 1.];
        let d = [0., 2.];
        let polygon = Polygon::new(Surface::xy_plane())
            .with_exterior(PolyChain::from([a, b, c, d]).close());
        assert_contains_point(polygon, [1., 1.]);

        // Ray hits a vertex, but doesn't pass polygon boundary there.
        let a = [0., 0.];
        let b = [2., 1.];
        let c = [3., 1.];
        let d = [4., 0.];
        let e = [4., 5.];
        let polygon = Polygon::new(Surface::xy_plane())
            .with_exterior(PolyChain::from([a, b, c, d, e]).close());
        assert_contains_point(polygon, [1., 1.]);
    }

    fn assert_contains_point(polygon: Polygon, point: impl Into<Point<2>>) {
        let point = point.into();

        assert!(polygon.contains_point(point, &mut DebugInfo::new()));
        assert!(polygon
            .invert_winding()
            .contains_point(point, &mut DebugInfo::new()));
    }
}
