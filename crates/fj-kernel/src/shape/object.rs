use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{validate::Validate, Handle, Mapping, Shape, ValidationResult};

/// Marker trait for geometric and topological objects
pub trait Object:
    'static + Clone + PartialEq + Validate + private::Sealed
{
    /// Internal function
    ///
    /// Please consider using [`Shape::merge`] instead.
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self>;
}

impl private::Sealed for Point<3> {}
impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge<3> {}
impl private::Sealed for Cycle<3> {}
impl private::Sealed for Face {}

impl Object for Point<3> {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let merged = shape.get_handle_or_insert(self)?;

        if let Some(handle) = handle {
            mapping.points.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Curve<3> {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let merged = shape.get_handle_or_insert(self)?;

        if let Some(handle) = handle {
            mapping.curves.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Surface {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let merged = shape.get_handle_or_insert(self)?;

        if let Some(handle) = handle {
            mapping.surfaces.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Vertex {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let point =
            self.point().merge_into(Some(self.point), shape, mapping)?;
        let merged = shape.get_handle_or_insert(Vertex { point })?;

        if let Some(handle) = handle {
            mapping.vertices.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Edge<3> {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let curve = self.curve().merge_into(
            Some(self.curve.canonical()),
            shape,
            mapping,
        )?;

        // Can be cleaned up using `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices = self.vertices.map(|vertices| {
            vertices.map(|vertex| {
                let canonical = vertex.canonical();
                canonical.get().merge_into(Some(canonical), shape, mapping)
            })
        });
        let vertices = match vertices {
            Some([a, b]) => Some([a?, b?]),
            None => None,
        };

        let merged = shape.get_handle_or_insert(Edge::new(curve, vertices))?;

        if let Some(handle) = handle {
            mapping.edges.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Cycle<3> {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let mut edges = Vec::new();
        for edge in self.edges {
            let edge = edge.canonical();
            let edge = edge.get().merge_into(Some(edge), shape, mapping)?;
            edges.push(edge);
        }

        let merged = shape.get_handle_or_insert(Cycle::new(edges))?;

        if let Some(handle) = handle {
            mapping.cycles.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

impl Object for Face {
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
        mapping: &mut Mapping,
    ) -> ValidationResult<Self> {
        let merged = match self {
            Face::Face(face) => {
                let surface = face.surface.get().merge_into(
                    Some(face.surface),
                    shape,
                    mapping,
                )?;

                let mut exts = Vec::new();
                for cycle in face.exteriors.as_handle() {
                    let merged =
                        cycle.get().merge_into(Some(cycle), shape, mapping)?;
                    exts.push(merged);
                }

                let mut ints = Vec::new();
                for cycle in face.interiors.as_handle() {
                    let merged =
                        cycle.get().merge_into(Some(cycle), shape, mapping)?;
                    ints.push(merged);
                }

                shape.get_handle_or_insert(Face::new(
                    surface, exts, ints, face.color,
                ))?
            }
            Face::Triangles(_) => shape.get_handle_or_insert(self)?,
        };

        if let Some(handle) = handle {
            mapping.faces.insert(handle, merged.clone());
        }

        Ok(merged)
    }
}

mod private {
    pub trait Sealed {}
}
