use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    objects::{GlobalEdge, HalfEdge, Surface},
    partial::{MaybeSurfacePath, Partial, PartialGlobalEdge, PartialHalfEdge},
};

use super::{CurveBuilder, VertexBuilder};

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
    fn update_from_other_edge(&mut self, other: &Partial<HalfEdge>);
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn replace_surface(&mut self, surface: impl Into<Partial<Surface>>) {
        let surface = surface.into();

        self.curve.write().surface = surface.clone();

        for vertex in &mut self.vertices {
            vertex.replace_surface(surface.clone());
        }
    }

    fn update_as_circle_from_radius(&mut self, radius: impl Into<Scalar>) {
        let path = self.curve.write().update_as_circle_from_radius(radius);

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let mut surface_vertex = {
            let [vertex, _] = &mut self.vertices;
            vertex.surface_form.clone()
        };
        surface_vertex.write().position =
            Some(path.point_from_path_coords(a_curve));

        for (vertex, point_curve) in
            self.vertices.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            let mut vertex = vertex;
            vertex.position = Some(point_curve);
            vertex.surface_form = surface_vertex.clone();
        }

        self.infer_global_form();
    }

    fn update_as_arc(&mut self, angle_rad: impl Into<Scalar>) {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }
        let points_surface = self.vertices.each_ref_ext().map(|vertex| {
            vertex
                .surface_form
                .read()
                .position
                .expect("Can't infer arc without surface position")
        });

        let arc = fj_math::Arc::from_endpoints_and_angle(
            points_surface[0],
            points_surface[1],
            angle_rad,
        );

        let path = self
            .curve
            .write()
            .update_as_circle_from_center_and_radius(arc.center, arc.radius);

        let [a_curve, b_curve] = if arc.flipped_construction {
            [arc.end_angle, arc.start_angle]
        } else {
            [arc.start_angle, arc.end_angle]
        }
        .map(|coord| Point::from([coord]));

        for (vertex, point_curve) in
            self.vertices.each_mut_ext().zip_ext([a_curve, b_curve])
        {
            vertex.position = Some(point_curve);
            vertex.surface_form.write().position =
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
            let mut surface_form = vertex.surface_form.write();
            surface_form.position = Some(point.into());
            surface_form.surface = surface.clone();
        }

        self.update_as_line_segment()
    }

    fn update_as_line_segment(&mut self) {
        let points_surface = self.vertices.each_ref_ext().map(|vertex| {
            vertex
                .surface_form
                .read()
                .position
                .expect("Can't infer line segment without surface position")
        });

        self.curve
            .write()
            .update_as_line_from_points(points_surface);

        for (vertex, position) in self.vertices.each_mut_ext().zip_ext([0., 1.])
        {
            vertex.position = Some([position].into());
        }

        self.infer_global_form();
    }

    fn infer_global_form(&mut self) -> Partial<GlobalEdge> {
        self.global_form.write().curve = self.curve.read().global_form.clone();
        self.global_form.write().vertices = self
            .vertices
            .each_ref_ext()
            .map(|vertex| vertex.surface_form.read().global_form.clone());

        self.global_form.clone()
    }

    fn update_from_other_edge(&mut self, other: &Partial<HalfEdge>) {
        let global_curve = other.read().curve.read().global_form.clone();
        self.curve.write().global_form = global_curve.clone();
        self.global_form.write().curve = global_curve;

        self.curve.write().path = other
            .read()
            .curve
            .read()
            .path
            .as_ref()
            .map(MaybeSurfacePath::to_undefined);

        for (this, other) in self
            .vertices
            .iter_mut()
            .zip(other.read().vertices.iter().rev())
        {
            this.surface_form.write().global_form.write().position =
                other.surface_form.read().global_form.read().position;
        }
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalEdgeBuilder for PartialGlobalEdge {}
