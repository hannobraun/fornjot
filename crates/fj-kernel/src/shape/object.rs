use crate::objects::{
    Curve, Cycle, Edge, Face, Surface, Vertex, VerticesOfEdge,
};

use super::{Handle, LocalForm, Shape};

/// Marker trait for geometric and topological objects
pub trait Object: 'static + Clone + PartialEq + private::Sealed {
    /// Internal function
    ///
    /// Please consider using [`Shape::merge`] instead.
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self>;
}

impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge<3> {}
impl private::Sealed for Cycle<3> {}
impl private::Sealed for Face {}

impl Object for Curve<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Surface {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Vertex {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        shape.get_handle_or_insert(Vertex { point: self.point })
    }
}

impl Object for Edge<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        let curve =
            self.curve().merge_into(Some(self.curve.canonical()), shape);

        let vertices = self.vertices.convert(|vertex| {
            let canonical = vertex.canonical();
            let canonical = canonical.get().merge_into(Some(canonical), shape);
            LocalForm::new(*vertex.local(), canonical)
        });

        shape.get_handle_or_insert(Edge {
            curve: LocalForm::canonical_only(curve),
            vertices: VerticesOfEdge::new(vertices),
        })
    }
}

impl Object for Cycle<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        let mut edges = Vec::new();
        for edge in self.edges {
            let edge = edge.canonical();
            let edge = edge.get().merge_into(Some(edge), shape);
            edges.push(edge);
        }

        shape.get_handle_or_insert(Cycle::new(edges))
    }
}

impl Object for Face {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> Handle<Self> {
        match self {
            Face::Face(face) => {
                let surface =
                    face.surface.get().merge_into(Some(face.surface), shape);

                let mut exts = Vec::new();
                for cycle in face.exteriors.as_local_form() {
                    let merged = cycle
                        .canonical()
                        .get()
                        .merge_into(Some(cycle.canonical()), shape);
                    exts.push(LocalForm::new(cycle.local().clone(), merged));
                }

                let mut ints = Vec::new();
                for cycle in face.interiors.as_local_form() {
                    let merged = cycle
                        .canonical()
                        .get()
                        .merge_into(Some(cycle.canonical()), shape);
                    ints.push(LocalForm::new(cycle.local().clone(), merged));
                }

                shape.get_handle_or_insert(Face::new(
                    surface, exts, ints, face.color,
                ))
            }
            Face::Triangles(_) => shape.get_handle_or_insert(self),
        }
    }
}

mod private {
    pub trait Sealed {}
}
