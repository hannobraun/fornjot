use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    partial::{Partial, PartialHalfEdge},
    services::Service,
};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder {
    /// Update partial half-edge to be a circle, from the given radius
    fn make_circle(
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Partial<HalfEdge>;

    /// Update partial half-edge to be an arc, spanning the given angle in
    /// radians
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    fn make_arc(
        start: impl Into<Point<2>>,
        end: impl Into<Point<2>>,
        angle_rad: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Partial<HalfEdge>;

    /// Update partial half-edge to be a line segment
    fn update_as_line_segment(
        &mut self,
        start: Point<2>,
        end: Point<2>,
    ) -> Curve;
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn make_circle(
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Partial<HalfEdge> {
        let curve = Curve::circle_from_radius(radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        Partial::from_partial(PartialHalfEdge {
            curve: Some(curve),
            boundary: boundary.map(Some),
            start_vertex: Vertex::new().insert(objects),
            global_form: GlobalEdge::new().insert(objects),
        })
    }

    fn make_arc(
        start: impl Into<Point<2>>,
        end: impl Into<Point<2>>,
        angle_rad: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Partial<HalfEdge> {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let curve =
            Curve::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        Partial::from_partial(PartialHalfEdge {
            curve: Some(curve),
            boundary: boundary.map(Some),
            start_vertex: Vertex::new().insert(objects),
            global_form: GlobalEdge::new().insert(objects),
        })
    }

    fn update_as_line_segment(
        &mut self,
        start: Point<2>,
        end: Point<2>,
    ) -> Curve {
        let points_surface = [start, end];

        let path = if let [Some(start), Some(end)] = self.boundary {
            let points = [start, end].zip_ext(points_surface);

            let path = Curve::line_from_points_with_coords(points);
            self.curve = Some(path);

            path
        } else {
            let (path, _) = Curve::line_from_points(points_surface);
            self.curve = Some(path);

            for (vertex, position) in
                self.boundary.each_mut_ext().zip_ext([0., 1.])
            {
                *vertex = Some([position].into());
            }

            path
        };

        path
    }
}
