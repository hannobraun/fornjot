use fj_math::Point;

use crate::{
    geometry::FloatingCurve,
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
    fn sweep(self, along: FloatingCurve, to: impl Into<Point<1>>) -> Solid;
}

impl SweepExt for Handle<Face> {
    fn sweep(self, along: FloatingCurve, to: impl Into<Point<1>>) -> Solid {
        let [from, to] = [Point::from([0.]), to.into()]
            .map(|point| along.vector_from_local_point(point));

        let bottom = self;
        let top = {
            let offset = to - from;
            Handle::new(bottom.flip().translate(offset))
        };

        top.connect(bottom, along)
    }
}
