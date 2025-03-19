use fj_math::Point;

use crate::mesh::Mesh;

/// An approximated model
#[derive(Clone, Debug)]
pub struct Model {
    /// The triangle mesh that approximates the model
    pub mesh: Mesh<Point<3>>,
}
