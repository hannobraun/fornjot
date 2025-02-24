use crate::{math::Vector, object::Handle, topology::face::Face};

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face;
}

impl TranslateExt for &Face {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face {
        let offset = offset.into();

        Face::new(
            self.surface().translate(offset),
            self.half_edges()
                .map(|vertex| Handle::new(vertex.start().translate(offset))),
        )
    }
}
