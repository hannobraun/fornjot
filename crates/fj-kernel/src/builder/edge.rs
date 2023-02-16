use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    geometry::path::{GlobalPath, SurfacePath},
    objects::{GlobalEdge, HalfEdge, Surface},
    partial::{MaybeSurfacePath, Partial, PartialGlobalEdge, PartialHalfEdge},
};

use super::CurveBuilder;

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder {
    /// Completely replace the surface in this half-edge's object graph
    ///
    /// Please note that this operation will write to both vertices that the
    /// half-edge references. If any of them were created from full objects,
    /// this will break the connection to those, meaning that building the
    /// partial objects won't result in those full objects again. This will be
    /// the case, even if those full objects already referenced the provided
    /// surface.
    fn replace_surface(&mut self, surface: impl Into<Partial<Surface>>);

    /// Update partial half-edge to be a circle, from the given radius
    fn update_as_circle_from_radius(&mut self, radius: impl Into<Scalar>);

    /// Update partial half-edge to be an arc, spanning the given angle in
    /// radians
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    fn update_as_arc(&mut self, angle_rad: impl Into<Scalar>);

    /// Update partial half-edge to be a line segment, from the given points
    fn update_as_line_segment_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: [impl Into<Point<2>>; 2],
    );

    /// Update partial half-edge to be a line segment
    fn update_as_line_segment(&mut self);

    /// Infer the global form of the half-edge
    ///
    /// Updates the global form referenced by this half-edge, and also returns
    /// it.
    fn infer_global_form(&mut self) -> Partial<GlobalEdge>;

    /// Update this edge from another
    ///
    /// Infers as much information about this edge from the other, under the
    /// assumption that the other edge is on a different surface.
    ///
    /// This method is quite fragile. It might panic, or even silently fail,
    /// under various circumstances. As long as you're only dealing with lines
    /// and planes, you should be fine. Otherwise, please read the code of this
    /// method carefully, to make sure you don't run into trouble.
    fn update_from_other_edge(&mut self, other: &Partial<HalfEdge>);
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn replace_surface(&mut self, surface: impl Into<Partial<Surface>>) {
        let surface = surface.into();

        self.curve.write().surface = surface.clone();

        for vertex in &mut self.vertices {
            vertex.1.write().surface = surface.clone();
        }
    }

    fn update_as_circle_from_radius(&mut self, radius: impl Into<Scalar>) {
        let path = self.curve.write().update_as_circle_from_radius(radius);

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let mut surface_vertex = {
            let [vertex, _] = &mut self.vertices;
            vertex.1.clone()
        };
        surface_vertex.write().position =
            Some(path.point_from_path_coords(a_curve));

        for (vertex, point_curve) in
            self.vertices.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            let mut vertex = vertex;
            vertex.0 = Some(point_curve);
            vertex.1 = surface_vertex.clone();
        }

        self.infer_global_form();
    }

    fn update_as_arc(&mut self, angle_rad: impl Into<Scalar>) {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }
        let [start, end] = self.vertices.each_ref_ext().map(|vertex| {
            vertex
                .1
                .read()
                .position
                .expect("Can't infer arc without surface position")
        });

        let arc = fj_math::Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path = self
            .curve
            .write()
            .update_as_circle_from_center_and_radius(arc.center, arc.radius);

        let [a_curve, b_curve] =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        for (vertex, point_curve) in
            self.vertices.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            vertex.0 = Some(point_curve);
            vertex.1.write().position =
                Some(path.point_from_path_coords(point_curve));
        }

        self.infer_global_form();
    }

    fn update_as_line_segment_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: [impl Into<Point<2>>; 2],
    ) {
        let surface = surface.into();

        self.curve.write().surface = surface.clone();

        for (vertex, point) in self.vertices.each_mut_ext().zip_ext(points) {
            let mut surface_form = vertex.1.write();
            surface_form.position = Some(point.into());
            surface_form.surface = surface.clone();
        }

        self.update_as_line_segment()
    }

    fn update_as_line_segment(&mut self) {
        let boundary = self.vertices.each_ref_ext().map(|vertex| vertex.0);
        let points_surface = self.vertices.each_ref_ext().map(|vertex| {
            vertex
                .1
                .read()
                .position
                .expect("Can't infer line segment without surface position")
        });

        if let [Some(start), Some(end)] = boundary {
            let boundary = [start, end];
            self.curve
                .write()
                .update_as_line_from_points_with_line_coords(
                    boundary.zip_ext(points_surface),
                );
        } else {
            self.curve
                .write()
                .update_as_line_from_points(points_surface);

            for (vertex, position) in
                self.vertices.each_mut_ext().zip_ext([0., 1.])
            {
                vertex.0 = Some([position].into());
            }
        }

        self.infer_global_form();
    }

    fn infer_global_form(&mut self) -> Partial<GlobalEdge> {
        self.global_form.write().curve = self.curve.read().global_form.clone();
        self.global_form.write().vertices = self
            .vertices
            .each_ref_ext()
            .map(|vertex| vertex.1.read().global_form.clone());

        self.global_form.clone()
    }

    fn update_from_other_edge(&mut self, other: &Partial<HalfEdge>) {
        let global_curve = other.read().curve.read().global_form.clone();
        self.curve.write().global_form = global_curve.clone();
        self.global_form.write().curve = global_curve;

        self.curve.write().path =
            other.read().curve.read().path.as_ref().and_then(|path| {
                match other.read().curve.read().surface.read().geometry {
                    Some(surface) => {
                        // We have information about the other edge's surface
                        // available. We need to use that to interpret what the
                        // other edge's curve path means for our curve path.
                        match surface.u {
                            GlobalPath::Circle(circle) => {
                                // The other surface is curved. We're entering
                                // some dodgy territory here, as only some edge
                                // cases can be represented using our current
                                // curve/surface representation.
                                match path {
                                    MaybeSurfacePath::Defined(
                                        SurfacePath::Line(_),
                                    )
                                    | MaybeSurfacePath::UndefinedLine => {
                                        // We're dealing with a line on a
                                        // rounded surface.
                                        //
                                        // Based on the current uses of this
                                        // method, we can make some assumptions:
                                        //
                                        // 1. The line is parallel to the u-axis
                                        //    of the other surface.
                                        // 2. The surface that *our* edge is in
                                        //    is a plane that is parallel to the
                                        //    the plane of the circle that
                                        //    defines the curvature of the other
                                        //    surface.
                                        //
                                        // These assumptions are necessary
                                        // preconditions for the following code
                                        // to work. But unfortunately, I see no
                                        // way to check those preconditions
                                        // here, as neither the other line nor
                                        // our surface is necessarily defined
                                        // yet.
                                        //
                                        // Handling this case anyway feels like
                                        // a grave sin, but I don't know what
                                        // else to do. If you tracked some
                                        // extremely subtle and annoying bug
                                        // back to this code, I apologize.
                                        //
                                        // I hope that I'll come up with a
                                        // better curve/surface representation
                                        // before this becomes a problem.
                                        Some(
                                            MaybeSurfacePath::UndefinedCircle {
                                                radius: circle.radius(),
                                            },
                                        )
                                    }
                                    _ => {
                                        // The other edge is a line segment in a
                                        // curved surface. No idea how to deal
                                        // with this.
                                        todo!(
                                            "Can't connect edge to circle on \
                                            curved surface"
                                        )
                                    }
                                }
                            }
                            GlobalPath::Line(_) => {
                                // The other edge is defined on a plane.
                                match path {
                                    MaybeSurfacePath::Defined(
                                        SurfacePath::Line(_),
                                    )
                                    | MaybeSurfacePath::UndefinedLine => {
                                        // The other edge is a line segment on
                                        // a plane. That means our edge must be
                                        // a line segment too.
                                        Some(MaybeSurfacePath::UndefinedLine)
                                    }
                                    _ => {
                                        // The other edge is a circle or arc on
                                        // a plane. I'm actually not sure what
                                        // that means for our edge. We might be
                                        // able to represent it somehow, but
                                        // let's leave that as an exercise for
                                        // later.
                                        todo!(
                                            "Can't connect edge to circle on \
                                            plane"
                                        )
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        // We know nothing about the surface the other edge is
                        // on. This means we can't infer anything about our
                        // curve from the other curve.
                        None
                    }
                }
            });

        for (this, other) in self
            .vertices
            .iter_mut()
            .zip(other.read().vertices.iter().rev())
        {
            this.1.write().global_form.write().position =
                other.1.read().global_form.read().position;
        }
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalEdgeBuilder for PartialGlobalEdge {}
