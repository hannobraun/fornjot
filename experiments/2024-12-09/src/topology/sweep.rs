use std::fmt;

use crate::{
    geometry::{AnyOp, Handle, Operation, TriMesh},
    math::Vector,
};

use super::{
    connect::{Connect, ConnectExt},
    face::Face,
    solid::Solid,
};

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
    fn sweep(self, path: impl Into<Vector<3>>) -> Sweep;
}

impl SweepExt for Handle<Face> {
    fn sweep(self, path: impl Into<Vector<3>>) -> Sweep {
        let bottom = self;
        let top = Handle::new(bottom.flip().translate(path));

        let output = top.connect(bottom);
        Sweep {
            output: Handle::new(output),
        }
    }
}

pub struct Sweep {
    output: Handle<Connect>,
}

impl Operation for Sweep {
    type Output = Solid;

    fn output(&self) -> &Self::Output {
        self.output.output()
    }

    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sweep")
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<AnyOp> {
        self.output.children()
    }
}
