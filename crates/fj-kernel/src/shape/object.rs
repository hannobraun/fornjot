use crate::objects::{
    Curve, Cycle, Edge, Face, Surface, Vertex, VerticesOfEdge,
};

use super::{Handle, LocalForm, Shape};

/// Marker trait for geometric and topological objects
pub trait Object: 'static + Clone + PartialEq + private::Sealed {
    /// Internal function
    ///
    /// Please consider using [`Shape::merge`] instead.
    fn merge_into(self, shape: &mut Shape) -> Handle<Self>;
}

impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge<3> {}
impl private::Sealed for Cycle<3> {}
impl private::Sealed for Face {}

impl Object for Curve<3> {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Surface {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Vertex {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        shape.get_handle_or_insert(Vertex { point: self.point })
    }
}

impl Object for Edge<3> {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        let curve = self.curve().merge_into(shape);

        let vertices = self.vertices.convert(|vertex| {
            let canonical = vertex.canonical();
            let canonical = canonical.merge_into(shape);
            LocalForm::new(*vertex.local(), canonical.get())
        });

        shape.get_handle_or_insert(Edge {
            curve: LocalForm::canonical_only(curve.get()),
            vertices: VerticesOfEdge::new(vertices),
        })
    }
}

impl Object for Cycle<3> {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        let mut edges = Vec::new();
        for edge in self.edges {
            let edge = edge.canonical().clone();
            let edge = edge.merge_into(shape);
            edges.push(edge.get());
        }

        shape.get_handle_or_insert(Cycle::new(edges))
    }
}

impl Object for Face {
    fn merge_into(self, shape: &mut Shape) -> Handle<Self> {
        match self {
            Face::Face(face) => {
                let surface = face.surface.merge_into(shape);

                let mut exts = Vec::new();
                for cycle in face.exteriors.as_local_form() {
                    let merged = cycle.canonical().clone().merge_into(shape);
                    exts.push(LocalForm::new(
                        cycle.local().clone(),
                        merged.get(),
                    ));
                }

                let mut ints = Vec::new();
                for cycle in face.interiors.as_local_form() {
                    let merged = cycle.canonical().clone().merge_into(shape);
                    ints.push(LocalForm::new(
                        cycle.local().clone(),
                        merged.get(),
                    ));
                }

                shape.get_handle_or_insert(Face::new(
                    surface.get(),
                    exts,
                    ints,
                    face.color,
                ))
            }
            Face::Triangles(_) => shape.get_handle_or_insert(self),
        }
    }
}

mod private {
    pub trait Sealed {}
}
