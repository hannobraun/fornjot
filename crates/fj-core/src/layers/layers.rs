use crate::{
    geometry::Geometry,
    presentation::Presentation,
    topology::Topology,
    validation::{Validation, ValidationConfig},
};

use super::Layer;

/// # Loosely coupled layers, that together define shapes
///
/// Shapes are not a monolithic thing in Fornjot, but instead are defined by
/// several, loosely coupled layers. These layers are owned by this struct.
///
/// ## Implementation Note
///
/// It is totally conceivable that one day, this system of layers is extensible
/// and more layers can be defined by third-party code. The foundation for that,
/// the loose coupling and inter-layer communication via events, is already
/// there, conceptually.
///
/// For now, there is no need for this, and all layers are just hardcoded here.
/// That can be changed, once necessary.
pub struct Layers {
    /// The topology layer
    ///
    /// Manages the stores of topological objects.
    pub topology: Layer<Topology>,

    /// The geometry layer
    ///
    /// Manages geometric information that applies to topological objects.
    pub geometry: Layer<Geometry>,

    /// The validation layer
    ///
    /// Monitors objects and validates them, as they are inserted.
    pub validation: Layer<Validation>,

    /// The presentation layer
    ///
    /// Stores data concerning the presentation of objects.
    pub presentation: Layer<Presentation>,
}

impl Layers {
    /// Construct an instance of `Layers`
    pub fn new() -> Self {
        let topology = Topology::new();
        let geometry = Geometry::new(&topology);

        Self {
            topology: Layer::new(topology),
            geometry: Layer::new(geometry),
            validation: Layer::default(),
            presentation: Layer::default(),
        }
    }

    /// Construct an instance of `Layers`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        Self {
            validation: Layer::new(Validation::with_validation_config(config)),
            ..Self::new()
        }
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}
