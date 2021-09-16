mod cell;
mod descriptor;
mod edge;
mod index;
mod surface_vertices;
mod vertex;

pub use self::{
    cell::Cell, descriptor::Descriptor, edge::Edge, index::Index,
    vertex::Vertex,
};

use std::collections::BTreeMap;

use nalgebra::Point;

use crate::geometry::attributes::{SignedDistanceField, SurfaceNormal};

use self::surface_vertices::SurfaceVertices;

use super::place_surface_vertex::place_surface_vertex;

// TASK: Don't use uniform grid when sampling geometry. Use an adaptive octree
//       instead (as described in the paper), to increase performance and reduce
//       memory use.

/// A uniform grid for isosurface extraction
#[derive(Debug)]
pub struct Grid {
    descriptor: Descriptor,
    all_edges: BTreeMap<(Index, Index), Edge>,
    edges_at_surface: BTreeMap<(Index, Index), Edge>,
    surface_vertices: SurfaceVertices,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: Descriptor,
        geometry: &(impl SignedDistanceField<3> + SurfaceNormal<3>),
    ) -> Self {
        let mut grid_vertex_samples = BTreeMap::new();

        let mut all_edges = BTreeMap::new();
        let mut edges_at_surface = BTreeMap::new();
        let mut surface_vertices = SurfaceVertices::new();

        for cell in descriptor.cells() {
            for index in cell.vertices() {
                let vertex = index
                    .to_position(descriptor.aabb.min, descriptor.resolution);
                grid_vertex_samples
                    .entry(index)
                    .or_insert_with(|| geometry.distance(vertex));
            }

            let mut points_and_normals = Vec::new();

            for (a, b) in cell.edges() {
                let sample_a = grid_vertex_samples[&a];
                let sample_b = grid_vertex_samples[&b];

                let edge = Edge {
                    a: Vertex {
                        index: a,
                        point: sample_a.point,
                        distance: sample_a.distance,
                    },
                    b: Vertex {
                        index: b,
                        point: sample_b.point,
                        distance: sample_b.distance,
                    },
                };

                all_edges.insert((a, b), edge);

                if edge.at_surface() {
                    edges_at_surface.insert((a, b), edge);

                    let f = edge.a.distance.abs()
                        / (edge.a.distance.abs() + edge.b.distance.abs());

                    assert!(f.is_finite());
                    assert!(!f.is_nan());

                    let point =
                        edge.a.point + (edge.b.point - edge.a.point) * f;
                    let normal = geometry.normal(point);

                    points_and_normals.push((point, normal));
                }
            }

            if points_and_normals.len() == 0 {
                continue;
            }

            let surface_vertex = place_surface_vertex(
                cell,
                descriptor.resolution,
                &points_and_normals,
            );

            surface_vertices.insert(cell.min_index, surface_vertex);
        }

        Self {
            descriptor,
            all_edges,
            edges_at_surface,
            surface_vertices,
        }
    }

    /// Access the grid descriptor
    pub fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }

    /// Iterate over all grid edges
    pub fn all_edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.all_edges.values().copied()
    }

    /// Iterate over all grid edges that are near a surface
    pub fn edges_at_surface(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges_at_surface.values().copied()
    }

    /// Get the 4 neighboring surface vertices of a grid edge
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        self.surface_vertices.neighbors_of_edge(edge)
    }
}
