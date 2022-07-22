use fj_math::{Transform, Vector};

use crate::{
    local::Local,
    objects::{
        Curve, Cycle, Edge, Face, GlobalVertex, Sketch, Solid, Surface, Vertex,
    },
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
    fn transform(self, transform: &Transform) -> Self;

    /// Translate the object
    #[must_use]
    fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        self.transform(&Transform::translation(offset))
    }

    /// Rotate the object
    #[must_use]
    fn rotate(self, axis_angle: impl Into<Vector<3>>) -> Self {
        self.transform(&Transform::rotation(axis_angle))
    }
}

impl TransformObject for Curve<3> {
    fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Circle(curve) => {
                Self::Circle(transform.transform_circle(&curve))
            }
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }
}

impl TransformObject for Cycle {
    fn transform(mut self, transform: &Transform) -> Self {
        for edge in &mut self.edges {
            *edge = edge.transform(transform);
        }

        self
    }
}

impl TransformObject for Edge {
    fn transform(self, transform: &Transform) -> Self {
        let curve = Local::new(
            *self.curve().local_form(),
            self.curve().global_form().transform(transform),
        );

        let vertices =
            self.vertices().map(|vertex| vertex.transform(transform));

        Self::new(curve, vertices)
    }
}

impl TransformObject for Face {
    fn transform(self, transform: &Transform) -> Self {
        if let Some(triangles) = self.triangles() {
            let mut target = Vec::new();

            for (triangle, color) in triangles.clone() {
                let triangle = transform.transform_triangle(&triangle);
                target.push((triangle, color));
            }

            return Self::from_triangles(target);
        }

        let surface = self.surface().transform(transform);

        let exteriors = transform_cycles(self.exteriors(), transform);
        let interiors = transform_cycles(self.interiors(), transform);

        let color = self.color();

        Face::new(surface)
            .with_exteriors(exteriors)
            .with_interiors(interiors)
            .with_color(color)
    }
}

impl TransformObject for GlobalVertex {
    fn transform(self, transform: &Transform) -> Self {
        let position = transform.transform_point(&self.position());
        Self::from_position(position)
    }
}

impl TransformObject for Sketch {
    fn transform(self, transform: &Transform) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform));
        Self::from_faces(faces)
    }
}

impl TransformObject for Solid {
    fn transform(self, transform: &Transform) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform));
        Self::from_faces(faces)
    }
}

impl TransformObject for Surface {
    fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::SweptCurve(surface) => {
                Self::SweptCurve(surface.transform(transform))
            }
        }
    }
}

impl TransformObject for Vertex {
    fn transform(self, transform: &Transform) -> Self {
        Self::new(self.position(), self.global().transform(transform))
    }
}

/// Transform a shape
pub fn transform_faces(faces: &mut Vec<Face>, transform: &Transform) {
    for face in faces {
        *face = face.clone().transform(transform);
    }
}

fn transform_cycles<'a>(
    cycles: impl IntoIterator<Item = &'a Cycle> + 'a,
    transform: &'a Transform,
) -> impl Iterator<Item = Cycle> + 'a {
    cycles
        .into_iter()
        .map(|cycle| cycle.clone().transform(transform))
}
