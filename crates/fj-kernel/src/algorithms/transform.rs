use fj_math::Transform;

use crate::{
    geometry::{Curve, Surface},
    shape::{Shape, ValidationError},
    topology::{Face, Vertex},
};

/// Transform the geometry of the shape
///
/// Since the topological types refer to geometry, and don't contain any
/// geometry themselves, this transforms the whole shape.
pub fn transform_shape(
    shape: &mut Shape,
    transform: &Transform,
) -> Result<(), ValidationError> {
    shape
        .update()
        .update_all(|vertex: &mut Vertex| {
            vertex.point = transform.transform_point(&vertex.point)
        })
        .update_all(|curve: &mut Curve<3>| *curve = curve.transform(transform))
        .update_all(|surface: &mut Surface| {
            *surface = surface.transform(transform)
        })
        .update_all(|mut face: &mut Face| {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.deref_mut() {
                for (triangle, _) in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        })
        .validate()?;

    Ok(())
}
