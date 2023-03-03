use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    geometry::{
        curve::{Curve, GlobalPath},
        surface::SurfaceGeometry,
    },
    objects::{HalfEdge, Vertex},
    partial::{MaybeCurve, Partial, PartialGlobalEdge, PartialHalfEdge},
};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder {
    /// Update partial half-edge to be a circle, from the given radius
    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> Curve;

    /// Update partial half-edge to be an arc, spanning the given angle in
    /// radians
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    fn update_as_arc(
        &mut self,
        start: Point<2>,
        end: Point<2>,
        angle_rad: impl Into<Scalar>,
    );

    /// Update partial half-edge to be a line segment
    fn update_as_line_segment(
        &mut self,
        start: Point<2>,
        end: Point<2>,
    ) -> Curve;

    /// Infer the vertex positions (surface and global), if not already set
    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
        next_vertex: Partial<Vertex>,
    );

    /// Update this edge from another
    ///
    /// Infers as much information about this edge from the other, under the
    /// assumption that the other edge is on a different surface.
    ///
    /// This method is quite fragile. It might panic, or even silently fail,
    /// under various circumstances. As long as you're only dealing with lines
    /// and planes, you should be fine. Otherwise, please read the code of this
    /// method carefully, to make sure you don't run into trouble.
    fn update_from_other_edge(
        &mut self,
        other: &Partial<HalfEdge>,
        other_prev: &Partial<HalfEdge>,
        surface: &SurfaceGeometry,
    );
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> Curve {
        let path = Curve::circle_from_radius(radius);
        self.curve = Some(path.into());

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        for (point_boundary, point_curve) in
            self.boundary.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            *point_boundary = Some(point_curve);
        }

        path
    }

    fn update_as_arc(
        &mut self,
        start: Point<2>,
        end: Point<2>,
        angle_rad: impl Into<Scalar>,
    ) {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = fj_math::Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path = Curve::circle_from_center_and_radius(arc.center, arc.radius);
        self.curve = Some(path.into());

        let [a_curve, b_curve] =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        for (point_boundary, point_curve) in
            self.boundary.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            *point_boundary = Some(point_curve);
        }
    }

    fn update_as_line_segment(
        &mut self,
        start: Point<2>,
        end: Point<2>,
    ) -> Curve {
        let boundary = self.boundary;
        let points_surface = [start, end];

        let path = if let [Some(start), Some(end)] = boundary {
            let points = [start, end].zip_ext(points_surface);

            let path = Curve::from_points_with_line_coords(points);
            self.curve = Some(path.into());

            path
        } else {
            let (path, _) = Curve::line_from_points(points_surface);
            self.curve = Some(path.into());

            for (vertex, position) in
                self.boundary.each_mut_ext().zip_ext([0., 1.])
            {
                *vertex = Some([position].into());
            }

            path
        };

        path
    }

    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
        next_vertex: Partial<Vertex>,
    ) {
        let path = self
            .curve
            .expect("Can't infer vertex positions without curve");
        let MaybeCurve::Defined(path) = path else {
            panic!("Can't infer vertex positions with undefined path");
        };

        for (boundary_point, mut vertex) in self
            .boundary
            .zip_ext([self.start_vertex.clone(), next_vertex])
        {
            let position_curve = boundary_point
                .expect("Can't infer surface position without curve position");
            let position_surface = path.point_from_path_coords(position_curve);

            // Infer global position, if not available.
            let position_global = vertex.read().position;
            if position_global.is_none() {
                let position_global =
                    surface.point_from_surface_coords(position_surface);
                vertex.write().position = Some(position_global);
            }
        }
    }

    fn update_from_other_edge(
        &mut self,
        other: &Partial<HalfEdge>,
        other_prev: &Partial<HalfEdge>,
        surface: &SurfaceGeometry,
    ) {
        self.curve = other.read().curve.as_ref().and_then(|path| {
            // We have information about the other edge's surface available. We
            // need to use that to interpret what the other edge's curve path
            // means for our curve path.
            match surface.u {
                GlobalPath::Circle(circle) => {
                    // The other surface is curved. We're entering some dodgy
                    // territory here, as only some edge cases can be
                    // represented using our current curve/surface
                    // representation.
                    match path {
                        MaybeCurve::Defined(Curve::Line(_))
                        | MaybeCurve::UndefinedLine => {
                            // We're dealing with a line on a rounded surface.
                            //
                            // Based on the current uses of this method, we can
                            // make some assumptions:
                            //
                            // 1. The line is parallel to the u-axis of the
                            //    other surface.
                            // 2. The surface that *our* edge is in is a plane
                            //    that is parallel to the the plane of the
                            //    circle that defines the curvature of the other
                            //    surface.
                            //
                            // These assumptions are necessary preconditions for
                            // the following code to work. But unfortunately, I
                            // see no way to check those preconditions here, as
                            // neither the other line nor our surface is
                            // necessarily defined yet.
                            //
                            // Handling this case anyway feels like a grave sin,
                            // but I don't know what else to do. If you tracked
                            // some extremely subtle and annoying bug back to
                            // this code, I apologize.
                            //
                            // I hope that I'll come up with a better curve/
                            // surface representation before this becomes a
                            // problem.
                            Some(MaybeCurve::UndefinedCircle {
                                radius: circle.radius(),
                            })
                        }
                        _ => {
                            // The other edge is a line segment in a curved
                            // surface. No idea how to deal with this.
                            todo!(
                                "Can't connect edge to circle on curved \
                                    surface"
                            )
                        }
                    }
                }
                GlobalPath::Line(_) => {
                    // The other edge is defined on a plane.
                    match path {
                        MaybeCurve::Defined(Curve::Line(_))
                        | MaybeCurve::UndefinedLine => {
                            // The other edge is a line segment on a plane. That
                            // means our edge must be a line segment too.
                            Some(MaybeCurve::UndefinedLine)
                        }
                        _ => {
                            // The other edge is a circle or arc on a plane. I'm
                            // actually not sure what that means for our edge.
                            // We might be able to represent it somehow, but
                            // let's leave that as an exercise for later.
                            todo!("Can't connect edge to circle on plane")
                        }
                    }
                }
            }
        });

        self.start_vertex.write().position =
            other_prev.read().start_vertex.read().position;
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalEdgeBuilder for PartialGlobalEdge {}
