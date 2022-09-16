//! Transforming objects

use fj_math::{Transform, Vector};

use crate::{
    objects::{
        Curve, Cycle, Face, Faces, GlobalCurve, GlobalVertex, HalfEdge, Shell,
        Sketch, Solid, Surface, SurfaceVertex, Vertex,
    },
    path::GlobalPath,
    stores::Stores,
};

/// Transform an object
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformObject: Sized {
    /// Transform the object
    #[must_use]
    fn transform(self, transform: &Transform, stores: &Stores) -> Self;

    /// Translate the object
    #[must_use]
    fn translate(self, offset: impl Into<Vector<3>>, stores: &Stores) -> Self {
        self.transform(&Transform::translation(offset), stores)
    }

    /// Rotate the object
    #[must_use]
    fn rotate(self, axis_angle: impl Into<Vector<3>>, stores: &Stores) -> Self {
        self.transform(&Transform::rotation(axis_angle), stores)
    }
}

impl TransformObject for Curve {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self.surface().transform(transform, stores);
        let global = self.global_form().transform(transform, stores);

        // Don't need to transform `self.kind`, as that's in local form.
        Curve::new(surface, self.path(), global)
    }
}

impl TransformObject for Cycle {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.surface().transform(transform, stores),
            self.into_half_edges()
                .map(|edge| edge.transform(transform, stores)),
        )
    }
}

impl TransformObject for Face {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self.surface().transform(transform, stores);

        let exterior = self.exterior().clone().transform(transform, stores);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().transform(transform, stores));

        let color = self.color();

        Face::new(surface, exterior)
            .with_interiors(interiors)
            .with_color(color)
    }
}

impl TransformObject for Faces {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let mut faces = Faces::new();
        faces.extend(
            self.into_iter()
                .map(|face| face.transform(transform, stores)),
        );
        faces
    }
}

impl TransformObject for GlobalCurve {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        GlobalCurve::from_path(self.path().transform(transform, stores))
    }
}

impl TransformObject for GlobalPath {
    fn transform(self, transform: &Transform, _: &Stores) -> Self {
        match self {
            Self::Circle(curve) => {
                Self::Circle(transform.transform_circle(&curve))
            }
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }
}

impl TransformObject for GlobalVertex {
    fn transform(self, transform: &Transform, _: &Stores) -> Self {
        let position = transform.transform_point(&self.position());
        Self::from_position(position)
    }
}

impl TransformObject for HalfEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let vertices = self
            .vertices()
            .clone()
            .map(|vertex| vertex.transform(transform, stores));

        Self::from_curve_and_vertices(curve, vertices)
    }
}

impl TransformObject for Shell {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform, stores));
        Self::new().with_faces(faces)
    }
}

impl TransformObject for Sketch {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform, stores));
        Self::new().with_faces(faces)
    }
}

impl TransformObject for Solid {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_shells()
            .map(|shell| shell.transform(transform, stores));
        Self::new().with_shells(faces)
    }
}

impl TransformObject for Surface {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.u().transform(transform, stores),
            transform.transform_vector(&self.v()),
        )
    }
}

impl TransformObject for SurfaceVertex {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.position(),
            self.surface().transform(transform, stores),
            self.global_form().transform(transform, stores),
        )
    }
}

impl TransformObject for Vertex {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.position(),
            self.curve().clone().transform(transform, stores),
            self.surface_form().transform(transform, stores),
            self.global_form().transform(transform, stores),
        )
    }
}
