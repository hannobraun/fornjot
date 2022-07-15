use fj_math::{Transform, Vector};

use crate::{
    local::Local,
    objects::{
        Curve, Cycle, CyclesInFace, Edge, Face, FaceBRep, GlobalVertex, Vertex,
    },
};

/// Extension trait that provides transformation methods for objects
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformExt: Sized {
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

impl TransformExt for Curve<3> {
    fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Circle(curve) => {
                Self::Circle(transform.transform_circle(&curve))
            }
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }
}

impl TransformExt for Cycle {
    fn transform(mut self, transform: &Transform) -> Self {
        for edge in &mut self.edges {
            *edge = edge.transform(transform);
        }

        self
    }
}

impl TransformExt for Edge {
    fn transform(self, transform: &Transform) -> Self {
        let curve = Local::new(
            self.curve.local(),
            self.curve.global().transform(transform),
        );

        let vertices = self
            .vertices
            .map(|vertex| transform_vertex(&vertex, transform));

        Self { curve, vertices }
    }
}

/// Transform a shape
pub fn transform_shape(faces: &mut Vec<Face>, transform: &Transform) {
    for face in faces {
        *face = transform_face(face, transform);
    }
}

pub fn transform_face(face: &Face, transform: &Transform) -> Face {
    match face {
        Face::Face(face) => {
            let surface = face.surface.transform(transform);

            let exteriors = transform_cycles(&face.exteriors, transform);
            let interiors = transform_cycles(&face.interiors, transform);

            let color = face.color;

            Face::Face(FaceBRep {
                surface,
                exteriors,
                interiors,
                color,
            })
        }
        Face::Triangles(triangles) => {
            let mut target = Vec::new();

            for &(triangle, color) in triangles {
                let triangle = transform.transform_triangle(&triangle);
                target.push((triangle, color));
            }

            Face::Triangles(target)
        }
    }
}

pub fn transform_cycles(
    cycles: &CyclesInFace,
    transform: &Transform,
) -> CyclesInFace {
    let cycles = cycles.as_local().map(|cycle| cycle.transform(transform));

    CyclesInFace::new(cycles)
}

pub fn transform_vertex(vertex: &Vertex, transform: &Transform) -> Vertex {
    let position = transform.transform_point(&vertex.global().position());
    let global = GlobalVertex::from_position(position);

    Vertex::new(vertex.position(), global)
}
