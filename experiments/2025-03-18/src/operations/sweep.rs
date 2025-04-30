use crate::{
    geometry::Line,
    handle::Handle,
    topology::{face::Face, solid::Solid},
};

use super::{connect::ConnectExt, flip::FlipExt, translate::TranslateExt};

pub trait SweepExt {
    /// # Sweep a face along a path, creating a solid
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
    fn sweep(self, along: Line) -> Solid;
}

impl SweepExt for Handle<Face> {
    fn sweep(self, along: Line) -> Solid {
        let bottom = self;
        let top = {
            let offset = along.direction;
            Handle::new(bottom.flip().translate(offset))
        };

        top.connect(bottom)
    }
}
