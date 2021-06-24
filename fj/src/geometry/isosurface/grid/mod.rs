mod cell;
mod descriptor;
mod edge;
mod index;
mod value;

pub use self::{
    cell::Cell, descriptor::Descriptor, edge::Edge, index::Index, value::Value,
};

use std::{array, collections::BTreeMap};

use nalgebra::{Point, Vector};

use crate::geometry::attributes::{Surface, SurfaceSample};

use self::edge::{Axis, Sign};

/// A uniform grid for isosurface extraction
#[derive(Debug)]
pub struct Grid {
    descriptor: Descriptor,
    grid_vertex_samples: GridVertexSamples,
    surface_vertices: BTreeMap<Index, Point<f32, 3>>,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: Descriptor,
        isosurface: &impl Surface<3>,
    ) -> Self {
        let surface_vertices = descriptor
            .cells()
            .map(|cell| {
                // We're saving the surface vertices of all grid cells here, but
                // we actually only need those that feature a sign change.

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

        let grid_vertex_samples = descriptor
            .vertices()
            .filter_map(|(index, vertex)| {
                // Compute distance of this vertex from the isosurface, and
                // filter all points that aren't close to the surface.
                let sample = isosurface.sample(vertex);
                if sample.distance <= descriptor.resolution {
                    Some((index, sample))
                } else {
                    None
                }
            })
            .collect();

        Self {
            descriptor,
            grid_vertex_samples,
            surface_vertices,
        }
    }

    /// Iterate over all grid edges that are near the surface
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.grid_vertex_samples
            .iter()
            .map(move |(&index, &sample)| {
                let next_z = [index.x(), index.y(), index.z() + 1];
                let next_y = [index.x(), index.y() + 1, index.z()];
                let next_x = [index.x() + 1, index.y(), index.z()];

                [
                    edge_to_next(
                        index,
                        sample,
                        next_z.into(),
                        &self.grid_vertex_samples,
                    ),
                    edge_to_next(
                        index,
                        sample,
                        next_y.into(),
                        &self.grid_vertex_samples,
                    ),
                    edge_to_next(
                        index,
                        sample,
                        next_x.into(),
                        &self.grid_vertex_samples,
                    ),
                ]
            })
            .map(|edges| array::IntoIter::new(edges))
            .flatten()
            .filter_map(|edge| edge)
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
            && edge.a.value < edge.b.value
            || direction.sign == Sign::Neg && edge.b.value < edge.a.value
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

fn edge_to_next(
    index: Index,
    sample: SurfaceSample<3>,
    next_index: Index,
    samples: &GridVertexSamples,
) -> Option<Edge> {
    let next_sample = samples.get(&next_index)?;

    Some(Edge {
        a: Value {
            index,
            point: sample.point,
            value: sample.distance,
        },
        b: Value {
            index: next_index,
            point: next_sample.point,
            value: next_sample.distance,
        },
    })
}

type GridVertexSamples = BTreeMap<Index, SurfaceSample<3>>;

#[cfg(test)]
mod tests {
    use nalgebra::Unit;

    use crate::geometry::{
        aabb::Aabb,
        attributes::{Surface, SurfaceSample},
        isosurface::grid,
    };

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
            &Geometry,
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
        let grid = Grid::from_descriptor(
            grid::Descriptor {
                aabb: Aabb {
                    min: [0.0, 0.0, 0.0].into(),
                    max: [1.0, 1.0, 1.0].into(),
                },
                resolution: 1.0,
            },
            &Geometry,
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
            &Geometry,
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
                        point: [1.0.into(), 1.0.into(), 1.0.into()].into(),
                        value: 1.0.into(),
                    },
                    b: grid::Value {
                        index: [2, 1, 1].into(),
                        point: [2.0.into(), 1.0.into(), 1.0.into()].into(),
                        value: 0.0.into(),
                    },
                },
                y: grid::Edge {
                    a: grid::Value {
                        index: [1, 1, 1].into(),
                        point: [1.0.into(), 1.0.into(), 1.0.into()].into(),
                        value: 1.0.into(),
                    },
                    b: grid::Value {
                        index: [1, 2, 1].into(),
                        point: [1.0.into(), 2.0.into(), 1.0.into()].into(),
                        value: 0.0.into(),
                    },
                },
                z: grid::Edge {
                    a: grid::Value {
                        index: [1, 1, 1].into(),
                        point: [1.0.into(), 1.0.into(), 1.0.into()].into(),
                        value: 1.0.into(),
                    },
                    b: grid::Value {
                        index: [1, 1, 2].into(),
                        point: [1.0.into(), 1.0.into(), 2.0.into()].into(),
                        value: 0.0.into(),
                    },
                },
            }
        }
    }

    struct Geometry;

    impl Surface<3> for Geometry {
        fn sample(
            &self,
            point: impl Into<nalgebra::Point<f32, 3>>,
        ) -> SurfaceSample<3> {
            let point = point.into();

            SurfaceSample {
                point,
                distance: 0.0,
                normal: Unit::new_normalize(point.coords),
            }
        }
    }
}
