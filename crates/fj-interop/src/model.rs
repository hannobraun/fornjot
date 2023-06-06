//! An approximated model

use fj_math::{Aabb, Point};

use crate::mesh::Mesh;

/// An approximated model
#[derive(Clone, Debug)]
pub struct Model {
    /// The triangle mesh that approximates the model
    pub mesh: Mesh<Point<3>>,

    /// The axis-aligned bounding box of the model
    pub aabb: Aabb<3>,
}
