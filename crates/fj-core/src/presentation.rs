//! Presentation data for the object graph
//!
//! See [`Presentation`].

use std::collections::BTreeMap;

use fj_interop::Color;

use crate::{objects::Region, storage::Handle};

/// Presentation data for the object graph
///
/// Assigns attributes relating to the presentation of objects (currently just a
/// color) to those objects (currently only to regions).
///
/// This data is made available through [`Layers`].
///
/// [`Layers`]: crate::layers::Layers
#[derive(Default)]
pub struct Presentation {
    /// Color assigned to regions
    ///
    /// Having a color is optional, so map does not necessarily contain
    /// assignments for all existing regions.
    pub color: BTreeMap<Handle<Region>, Color>,
}
