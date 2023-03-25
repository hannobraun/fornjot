use crate::Shape;

use super::Metadata;

/// A serialized model that can be passed from client code to a host.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Model {
    /// Metadata for the model.
    pub metadata: Metadata,

    /// This model's concrete geometry.
    pub shape: Shape,
}
