use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex, VerticesOfEdge},
};

use super::{
    validate::Validate, Handle, LocalForm, Mapping, Shape, ValidationError,
    ValidationResult,
};

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

impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge<3> {}
impl private::Sealed for Cycle<3> {}
impl private::Sealed for Face {}

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
        let merged =
            shape.get_handle_or_insert(Vertex { point: self.point })?;

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

        let vertices =
            self.vertices
                .try_convert::<_, _, ValidationError>(|vertex| {
                    let canonical = vertex.canonical();
                    let canonical = canonical.get().merge_into(
                        Some(canonical),
                        shape,
                        mapping,
                    )?;
                    Ok(LocalForm::new(*vertex.local(), canonical))
                })?;

        let merged = shape.get_handle_or_insert(Edge::new(
            curve,
            VerticesOfEdge::new(vertices),
        ))?;

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
                for cycle in face.exteriors.as_local_form() {
                    let merged = cycle.canonical().get().merge_into(
                        Some(cycle.canonical()),
                        shape,
                        mapping,
                    )?;
                    exts.push(LocalForm::new(cycle.local().clone(), merged));
                }

                let mut ints = Vec::new();
                for cycle in face.interiors.as_local_form() {
                    let merged = cycle.canonical().get().merge_into(
                        Some(cycle.canonical()),
                        shape,
                        mapping,
                    )?;
                    ints.push(LocalForm::new(cycle.local().clone(), merged));
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
