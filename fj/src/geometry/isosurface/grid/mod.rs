mod cell;
mod descriptor;
mod edge;
mod index;
mod surface_vertices;
mod value;

pub use self::{
    cell::Cell, descriptor::Descriptor, edge::Edge, index::Index, value::Value,
};

use std::collections::BTreeMap;

use nalgebra::Point;

use crate::geometry::attributes::Surface;

use self::surface_vertices::SurfaceVertices;

/// A uniform grid for isosurface extraction
#[derive(Debug)]
pub struct Grid {
    descriptor: Descriptor,
    edges: BTreeMap<(Index, Index), Edge>,
    surface_vertices: SurfaceVertices,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: Descriptor,
        isosurface: &impl Surface<3>,
    ) -> Self {
        let mut grid_vertex_samples = BTreeMap::new();
        let mut edges = BTreeMap::new();

        let surface_vertices = descriptor
            .cells()
            .filter_map(|cell| {
                for (index, vertex) in cell.vertices(descriptor.resolution) {
                    grid_vertex_samples
                        .entry(index)
                        .or_insert(isosurface.sample(vertex));
                }

                let mut surface_vertex = Point::origin();
                let mut num_edges_at_surface = 0;

                for (a, b) in cell.edges() {
                    let sample_a = grid_vertex_samples[&a];
                    let sample_b = grid_vertex_samples[&b];

                    // Since neighboring cells share edges, we're duplicating
                    // their creation here, overwriting previous results, should
                    // they exist.
                    //
                    // This shouldn't change anything about the result, but it's
                    // extra work. It might be better to check whether an edge
                    // is already available and use that.

                    let edge = Edge {
                        a: Value {
                            index: a,
                            point: sample_a.point,
                            distance: sample_a.distance,
                        },
                        b: Value {
                            index: b,
                            point: sample_b.point,
                            distance: sample_b.distance,
                        },
                    };

                    if edge.at_surface() {
                        edges.insert((a, b), edge);
                        num_edges_at_surface += 1;

                        let f = edge.a.distance.abs()
                            / (edge.a.distance.abs() + edge.b.distance.abs());

                        assert!(f.is_finite());
                        assert!(!f.is_nan());

                        let point =
                            edge.a.point + (edge.b.point - edge.a.point) * f;

                        surface_vertex += point.coords;
                    }
                }

                if num_edges_at_surface == 0 {
                    return None;
                }

                // We just average all of the points that intersect the surface,
                // discarding surface normals. This is simpler than the method
                // described in "Dual Contouring of Hermite Data".
                surface_vertex /= num_edges_at_surface as f32;

                Some((cell.min_index, surface_vertex))
            })
            .collect();

        Self {
            descriptor,
            edges,
            surface_vertices: SurfaceVertices(surface_vertices),
        }
    }

    /// Iterate over all grid edges that are near a surface
    pub fn edges_at_surface(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.values().copied()
    }

    /// Returns the 4 neighboring surface vertices of a grid edge
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        self.surface_vertices.neighbors_of_edge(edge)
    }
}
