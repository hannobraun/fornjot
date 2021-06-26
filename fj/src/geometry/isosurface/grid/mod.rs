mod cell;
mod descriptor;
mod edge;
mod index;
mod value;

pub use self::{
    cell::Cell, descriptor::Descriptor, edge::Edge, index::Index, value::Value,
};

use std::collections::BTreeMap;

use nalgebra::{Point, Vector};

use crate::geometry::attributes::Surface;

use self::edge::{Axis, Sign};

/// A uniform grid for isosurface extraction
#[derive(Debug)]
pub struct Grid {
    descriptor: Descriptor,
    edges: BTreeMap<(Index, Index), Edge>,
    surface_vertices: BTreeMap<Index, Point<f32, 3>>,
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
            .map(|cell| {
                // We're saving the grid vertex samples and surface vertices of
                // all grid cells here, but we actually only need those for
                // cells that feature a sign change.

                for (index, vertex) in cell.vertices(descriptor.resolution) {
                    // Since neighboring cells share vertices, we're duplicating
                    // lots of computations here, overwriting previous results,
                    // if they exist.
                    //
                    // This shouldn't change anything about the result, but it's
                    // lots of extra work. It would be better to check whether a
                    // result is already available and use that.
                    let sample = isosurface.sample(vertex);
                    grid_vertex_samples.insert(index, sample);
                }

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

                    // We're storing _all_ edges, but we actually only need
                    // those that are at a surface.
                    edges.insert((a, b), edge);
                }

                // TASK: Place surface vertex more accurately by minimizing the
                //       error function as per the paper, section 2.3.
                let surface_vertex = cell.min_position
                    + Vector::from([
                        descriptor.resolution / 2.0,
                        descriptor.resolution / 2.0,
                        descriptor.resolution / 2.0,
                    ]);

                (cell.min_index, surface_vertex)
            })
            .collect();

        Self {
            descriptor,
            edges,
            surface_vertices,
        }
    }

    /// Iterate over all grid edges that are near the surface
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.values().copied()
    }

    /// Returns the 4 neighboring surface vertices of a grid edge
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        let direction = edge.direction();

        #[rustfmt::skip]
        let [a, b, c, d] = match direction.axis {
            Axis::Z => [
                [ 0, -1, 0],
                [-1, -1, 0],
                [-1,  0, 0],
                [ 0,  0, 0],
            ],
            Axis::Y => [
                [-1, 0, -1],
                [ 0, 0, -1],
                [ 0, 0,  0],
                [-1, 0,  0],
            ],
            Axis::X => [
                [0,  0, -1],
                [0, -1, -1],
                [0, -1,  0],
                [0,  0,  0],
            ],
        };

        let start = match direction.sign {
            Sign::Neg => edge.b,
            Sign::Pos => edge.a,
        };
        let start = start.index;

        let [a, b, c, d] = if direction.sign == Sign::Pos
            && edge.a.distance < edge.b.distance
            || direction.sign == Sign::Neg && edge.b.distance < edge.a.distance
        {
            [b, a, d, c]
        } else {
            [a, b, c, d]
        };

        let neighbors = [
            self.surface_vertices[&(start + a)],
            self.surface_vertices[&(start + b)],
            self.surface_vertices[&(start + c)],
            self.surface_vertices[&(start + d)],
        ];

        neighbors
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::{aabb::Aabb, isosurface::grid, shapes::Sphere};

    use super::Grid;

    #[test]
    fn edges_should_return_edges() {
        let grid = Grid::from_descriptor(
            grid::Descriptor {
                aabb: Aabb {
                    min: [0.0, 0.0, 0.0].into(),
                    max: [1.0, 1.0, 1.0].into(),
                },
                resolution: 1.0,
            },
            &Sphere::new(),
        );

        let edges: Vec<_> = grid
            .edges()
            .map(|edge| (edge.a.index, edge.b.index))
            .collect();

        assert_eq!(
            edges,
            vec![
                ([0, 0, 0].into(), [0, 0, 1].into()),
                ([0, 0, 0].into(), [0, 1, 0].into()),
                ([0, 0, 0].into(), [1, 0, 0].into()),
                ([0, 0, 1].into(), [0, 0, 2].into()),
                ([0, 0, 1].into(), [0, 1, 1].into()),
                ([0, 0, 1].into(), [1, 0, 1].into()),
                ([0, 0, 2].into(), [0, 1, 2].into()),
                ([0, 0, 2].into(), [1, 0, 2].into()),
                ([0, 1, 0].into(), [0, 1, 1].into()),
                ([0, 1, 0].into(), [0, 2, 0].into()),
                ([0, 1, 0].into(), [1, 1, 0].into()),
                ([0, 1, 1].into(), [0, 1, 2].into()),
                ([0, 1, 1].into(), [0, 2, 1].into()),
                ([0, 1, 1].into(), [1, 1, 1].into()),
                ([0, 1, 2].into(), [0, 2, 2].into()),
                ([0, 1, 2].into(), [1, 1, 2].into()),
                ([0, 2, 0].into(), [0, 2, 1].into()),
                ([0, 2, 0].into(), [1, 2, 0].into()),
                ([0, 2, 1].into(), [0, 2, 2].into()),
                ([0, 2, 1].into(), [1, 2, 1].into()),
                ([0, 2, 2].into(), [1, 2, 2].into()),
                ([1, 0, 0].into(), [1, 0, 1].into()),
                ([1, 0, 0].into(), [1, 1, 0].into()),
                ([1, 0, 0].into(), [2, 0, 0].into()),
                ([1, 0, 1].into(), [1, 0, 2].into()),
                ([1, 0, 1].into(), [1, 1, 1].into()),
                ([1, 0, 1].into(), [2, 0, 1].into()),
                ([1, 0, 2].into(), [1, 1, 2].into()),
                ([1, 0, 2].into(), [2, 0, 2].into()),
                ([1, 1, 0].into(), [1, 1, 1].into()),
                ([1, 1, 0].into(), [1, 2, 0].into()),
                ([1, 1, 0].into(), [2, 1, 0].into()),
                ([1, 1, 1].into(), [1, 1, 2].into()),
                ([1, 1, 1].into(), [1, 2, 1].into()),
                ([1, 1, 1].into(), [2, 1, 1].into()),
                ([1, 1, 2].into(), [1, 2, 2].into()),
                ([1, 1, 2].into(), [2, 1, 2].into()),
                ([1, 2, 0].into(), [1, 2, 1].into()),
                ([1, 2, 0].into(), [2, 2, 0].into()),
                ([1, 2, 1].into(), [1, 2, 2].into()),
                ([1, 2, 1].into(), [2, 2, 1].into()),
                ([1, 2, 2].into(), [2, 2, 2].into()),
                ([2, 0, 0].into(), [2, 0, 1].into()),
                ([2, 0, 0].into(), [2, 1, 0].into()),
                ([2, 0, 1].into(), [2, 0, 2].into()),
                ([2, 0, 1].into(), [2, 1, 1].into()),
                ([2, 0, 2].into(), [2, 1, 2].into()),
                ([2, 1, 0].into(), [2, 1, 1].into()),
                ([2, 1, 0].into(), [2, 2, 0].into()),
                ([2, 1, 1].into(), [2, 1, 2].into()),
                ([2, 1, 1].into(), [2, 2, 1].into()),
                ([2, 1, 2].into(), [2, 2, 2].into()),
                ([2, 2, 0].into(), [2, 2, 1].into()),
                ([2, 2, 1].into(), [2, 2, 2].into()),
            ]
        );
    }

    #[test]
    fn neighbors_of_edge_should_return_neighboring_grid_centers() {
        let geometry = Sphere::new();

        let grid = Grid::from_descriptor(
            grid::Descriptor {
                aabb: Aabb {
                    min: [0.0, 0.0, 0.0].into(),
                    max: [1.0, 1.0, 1.0].into(),
                },
                resolution: 1.0,
            },
            &geometry,
        );

        let edges = TestEdges::new();

        let [x0, x1, x2, x3] = [
            [1.0, 1.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];
        let [y0, y1, y2, y3] = [
            [0.0, 1.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
        ];
        let [z0, z1, z2, z3] = [
            [1.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];

        assert_eq!(grid.neighbors_of_edge(edges.x), [x0, x1, x2, x3]);
        assert_eq!(grid.neighbors_of_edge(edges.y), [y0, y1, y2, y3]);
        assert_eq!(grid.neighbors_of_edge(edges.z), [z0, z1, z2, z3]);

        assert_eq!(
            grid.neighbors_of_edge(edges.x.swap_values()),
            [x1, x0, x3, x2],
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.y.swap_values()),
            [y1, y0, y3, y2],
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.z.swap_values()),
            [z1, z0, z3, z2],
        );
    }

    #[test]
    fn neighbors_of_edge_should_work_regardless_of_direction() {
        let grid = Grid::from_descriptor(
            grid::Descriptor {
                aabb: Aabb {
                    min: [0.0, 0.0, 0.0].into(),
                    max: [1.0, 1.0, 1.0].into(),
                },
                resolution: 1.0,
            },
            &Sphere::new(),
        );

        let edges = TestEdges::new();

        assert_eq!(
            grid.neighbors_of_edge(edges.x),
            grid.neighbors_of_edge(edges.x.reverse()),
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.y),
            grid.neighbors_of_edge(edges.y.reverse()),
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.z),
            grid.neighbors_of_edge(edges.z.reverse()),
        );
    }

    pub struct TestEdges {
        pub x: grid::Edge,
        pub y: grid::Edge,
        pub z: grid::Edge,
    }

    impl TestEdges {
        pub fn new() -> Self {
            Self {
                x: grid::Edge {
                    a: grid::Value {
                        index: [1, 1, 1].into(),
                        point: [1.0, 1.0, 1.0].into(),
                        distance: 1.0,
                    },
                    b: grid::Value {
                        index: [2, 1, 1].into(),
                        point: [2.0, 1.0, 1.0].into(),
                        distance: 0.0,
                    },
                },
                y: grid::Edge {
                    a: grid::Value {
                        index: [1, 1, 1].into(),
                        point: [1.0, 1.0, 1.0].into(),
                        distance: 1.0,
                    },
                    b: grid::Value {
                        index: [1, 2, 1].into(),
                        point: [1.0, 2.0, 1.0].into(),
                        distance: 0.0,
                    },
                },
                z: grid::Edge {
                    a: grid::Value {
                        index: [1, 1, 1].into(),
                        point: [1.0, 1.0, 1.0].into(),
                        distance: 1.0,
                    },
                    b: grid::Value {
                        index: [1, 1, 2].into(),
                        point: [1.0, 1.0, 2.0].into(),
                        distance: 0.0,
                    },
                },
            }
        }
    }
}
