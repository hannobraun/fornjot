use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::{
    math::Point,
    object::Handle,
    topology::{
        face::Face, half_edge::HalfEdge, surface::Surface, vertex::Vertex,
    },
};

/// # A 2D sketch, which one way to create faces
///
/// So far, sketches are pretty limited: They are just a bunch of ordered
/// points. Those points can be converted into the straight half-edges that
/// bound a face.
///
/// You could create this struct manually, but there's also a [`From`]
/// implementation that can create an instance of this struct from any iterator
/// that yields points.
///
/// The next step here, would be to add support for curved edges. But this would
/// need to be supported on the topology side first.
pub struct Sketch {
    pub points: Vec<Point<2>>,
}

impl Sketch {
    /// # Convert the sketch into a face
    ///
    /// The `surface` parameter defines the plane which is then used to convert
    /// the 2D sketch into a 3D face. In the future, more surfaces than just
    /// planes would be supported, but we're not there yet.
    pub fn to_face(&self, surface: Handle<Surface>) -> Face {
        let mut vertices_by_local_point: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let vertices = self
            .points
            .iter()
            .copied()
            .map(|point| {
                let point = surface.geometry.point_from_local(point);
                let vertex = Handle::new(Vertex::new(point));

                vertices_by_local_point
                    .entry(point)
                    .or_default()
                    .push(vertex.clone());

                vertex
            })
            .collect::<Vec<_>>();

        let mut coincident_vertices = BTreeSet::new();
        for vertices in vertices_by_local_point.into_values() {
            if vertices.len() > 1 {
                coincident_vertices.extend(vertices);
            }
        }

        let half_edges = vertices.into_iter().circular_tuple_windows().map(
            |(start, end)| {
                let is_internal = coincident_vertices.contains(&start)
                    && coincident_vertices.contains(&end);

                Handle::new(HalfEdge { start, is_internal })
            },
        );

        Face::new(surface, half_edges, false)
    }
}

impl<I, P> From<I> for Sketch
where
    I: IntoIterator<Item = P>,
    P: Into<Point<2>>,
{
    fn from(points: I) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}
