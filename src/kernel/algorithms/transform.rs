use crate::{kernel::topology::Shape, math::Transform};

/// Create a new shape that is a transformed version of an existing one
///
/// # Implementation note
///
/// This code isn't really correct, only transforming the faces of the original
/// shape and not taking care of anything else, but this is more a reflection of
/// the state of `Shape`, with its redundant data.
///
/// Addressing the shortcomings in this method probably doesn't make sense,
/// except as a side effect of addressing the shortcomings of `Shape`.
pub fn transform_shape(original: &Shape, transform: &Transform) -> Shape {
    let mut transformed = Shape::new();
    transformed.faces = original.faces.clone().transform(transform);
    transformed
}
