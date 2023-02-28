use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    geometry::{
        curve::{Curve, GlobalPath},
        surface::SurfaceGeometry,
    },
    objects::{GlobalEdge, HalfEdge, SurfaceVertex},
    partial::{MaybeCurve, Partial, PartialGlobalEdge, PartialHalfEdge},
};

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder {
    /// Update partial half-edge to represent the u-axis of the surface it is on
    ///
    /// Returns the updated path.
    fn update_as_u_axis(&mut self) -> Curve;

    /// Update partial curve to represent the v-axis of the surface it is on
    ///
    /// Returns the updated path.
    fn update_as_v_axis(&mut self) -> Curve;

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
        angle_rad: impl Into<Scalar>,
        next_vertex: Partial<SurfaceVertex>,
    );

    /// Update partial half-edge to be a line segment, from the given points
    fn update_as_line_segment_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
        next_vertex: Partial<SurfaceVertex>,
    ) -> Curve;

    /// Update partial half-edge to be a line segment
    fn update_as_line_segment(
        &mut self,
        next_vertex: Partial<SurfaceVertex>,
    ) -> Curve;

    /// Infer the global form of the half-edge
    ///
    /// Updates the global form referenced by this half-edge, and also returns
    /// it.
    fn infer_global_form(
        &mut self,
        next_vertex: Partial<SurfaceVertex>,
    ) -> Partial<GlobalEdge>;

    /// Infer the vertex positions (surface and global), if not already set
    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
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
        surface: &SurfaceGeometry,
    );
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn update_as_u_axis(&mut self) -> Curve {
        let path = Curve::u_axis();
        self.curve = Some(path.into());
        path
    }

    fn update_as_v_axis(&mut self) -> Curve {
        let path = Curve::v_axis();
        self.curve = Some(path.into());
        path
    }

    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> Curve {
        let path = Curve::circle_from_radius(radius);
        self.curve = Some(path.into());

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        self.start_vertex.write().position =
            Some(path.point_from_path_coords(a_curve));
        self.end_vertex = self.start_vertex.clone();

        for (point_boundary, point_curve) in
            self.boundary.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            *point_boundary = Some(point_curve);
        }

        self.infer_global_form(self.start_vertex.clone());

        path
    }

    fn update_as_arc(
        &mut self,
        angle_rad: impl Into<Scalar>,
        mut next_vertex: Partial<SurfaceVertex>,
    ) {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }
        let [start, end] = [&self.start_vertex, &next_vertex].map(|vertex| {
            vertex
                .read()
                .position
                .expect("Can't infer arc without surface position")
        });

        let arc = fj_math::Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path = Curve::circle_from_center_and_radius(arc.center, arc.radius);
        self.curve = Some(path.into());

        let [a_curve, b_curve] =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        for ((point_boundary, surface_vertex), point_curve) in self
            .boundary
            .each_mut_ext()
            .zip_ext([&mut self.start_vertex, &mut next_vertex])
            .zip_ext([a_curve, b_curve])
        {
            *point_boundary = Some(point_curve);
            surface_vertex.write().position =
                Some(path.point_from_path_coords(point_curve));
        }

        self.infer_global_form(next_vertex);
    }

    fn update_as_line_segment_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
        mut next_vertex: Partial<SurfaceVertex>,
    ) -> Curve {
        for (vertex, point) in
            [&mut self.start_vertex, &mut next_vertex].zip_ext(points)
        {
            let mut surface_form = vertex.write();
            surface_form.position = Some(point.into());
        }

        self.update_as_line_segment(next_vertex)
    }

    fn update_as_line_segment(
        &mut self,
        next_vertex: Partial<SurfaceVertex>,
    ) -> Curve {
        let boundary = self.boundary;
        let points_surface = [&self.start_vertex, &next_vertex].map(|vertex| {
            vertex
                .read()
                .position
                .expect("Can't infer line segment without surface position")
        });

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

        self.infer_global_form(next_vertex);

        path
    }

    fn infer_global_form(
        &mut self,
        next_vertex: Partial<SurfaceVertex>,
    ) -> Partial<GlobalEdge> {
        self.global_form.write().vertices = [&self.start_vertex, &next_vertex]
            .map(|vertex| vertex.read().global_form.clone());

        self.global_form.clone()
    }

    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    ) {
        let path = self
            .curve
            .expect("Can't infer vertex positions without curve");
        let MaybeCurve::Defined(path) = path else {
            panic!("Can't infer vertex positions with undefined path");
        };

        for (boundary_point, surface_vertex) in self
            .boundary
            .zip_ext([&mut self.start_vertex, &mut self.end_vertex])
        {
            let position_curve = boundary_point
                .expect("Can't infer surface position without curve position");

            let position_surface = surface_vertex.read().position;

            // Infer surface position, if not available.
            let position_surface = match position_surface {
                Some(position_surface) => position_surface,
                None => {
                    let position_surface =
                        path.point_from_path_coords(position_curve);

                    surface_vertex.write().position = Some(position_surface);

                    position_surface
                }
            };

            // Infer global position, if not available.
            let position_global =
                surface_vertex.read().global_form.read().position;
            if position_global.is_none() {
                let position_global =
                    surface.point_from_surface_coords(position_surface);
                surface_vertex.write().global_form.write().position =
                    Some(position_global);
            }
        }
    }

    fn update_from_other_edge(
        &mut self,
        other: &Partial<HalfEdge>,
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

        let other = other.read();

        for (this, other) in [&mut self.start_vertex, &mut self.end_vertex]
            .iter_mut()
            .zip([&other.end_vertex, &other.start_vertex])
        {
            this.write().global_form.write().position =
                other.read().global_form.read().position;
        }
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalEdgeBuilder for PartialGlobalEdge {}
