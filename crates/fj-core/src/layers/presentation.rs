//! Layer infrastructure for [`Presentation`]

use fj_interop::Color;

use crate::{
    presentation::Presentation,
    storage::Handle,
    topology::{AnyObject, Region, Stored},
};

use super::{Command, Event, Layer};

impl Layer<Presentation> {
    /// Set the color of a region
    pub fn set_color(&mut self, region: Handle<Region>, color: Color) {
        self.process_command(SetColor { region, color });
    }

    /// Mark an object as being derived from another
    pub fn derive_object(
        &mut self,
        original: AnyObject<Stored>,
        derived: AnyObject<Stored>,
    ) {
        self.process_command(DeriveObject { original, derived });
    }
}

/// Set the color of a region
pub struct SetColor {
    /// The region to set the color for
    region: Handle<Region>,

    /// The color to set
    color: Color,
}

impl Command<Presentation> for SetColor {
    type Result = ();
    type Event = Self;

    fn decide(
        self,
        _: &Presentation,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        events.push(self);
    }
}

impl Event<Presentation> for SetColor {
    fn evolve(&self, state: &mut Presentation) {
        state.color.insert(self.region.clone(), self.color);
    }
}

/// Handle an object being derived from another
pub struct DeriveObject {
    /// The original object
    original: AnyObject<Stored>,

    /// The derived object
    derived: AnyObject<Stored>,
}

impl Command<Presentation> for DeriveObject {
    type Result = ();
    type Event = SetColor;

    fn decide(
        self,
        state: &Presentation,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        if let (AnyObject::Region(original), AnyObject::Region(derived)) =
            (self.original, self.derived)
        {
            if let Some(color) = state.color.get(&original).cloned() {
                events.push(SetColor {
                    region: derived,
                    color,
                });
            }
        }
    }
}

/// Event produced by `Layer<Presentation>`
#[derive(Clone)]
pub enum PresentationEvent {
    /// The color of a region is being set
    SetColor {
        /// The region the color is being set for
        region: Handle<Region>,

        /// The color being set
        color: Color,
    },
}
