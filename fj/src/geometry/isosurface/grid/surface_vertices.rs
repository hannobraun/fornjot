use std::collections::BTreeMap;

use nalgebra::Point;

use super::{
    edge::{Axis, Sign},
    Edge, Index,
};

#[derive(Debug)]
pub struct SurfaceVertices(pub BTreeMap<Index, Point<f32, 3>>);

impl SurfaceVertices {
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
            self.0[&(start + a)],
            self.0[&(start + b)],
            self.0[&(start + c)],
            self.0[&(start + d)],
        ];

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use nalgebra::Point;

    use crate::geometry::{
        aabb::Aabb,
        isosurface::{grid, Grid},
        shapes::Sphere,
    };

    use super::SurfaceVertices;

    #[test]
    fn neighbors_of_edge_should_return_neighboring_surface_vertices() {
        let surface_vertices = SurfaceVertices(test_vertices());

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

        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.x),
            [x0, x1, x2, x3]
        );
        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.y),
            [y0, y1, y2, y3]
        );
        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.z),
            [z0, z1, z2, z3]
        );

        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.x.swap_values()),
            [x1, x0, x3, x2],
        );
        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.y.swap_values()),
            [y1, y0, y3, y2],
        );
        assert_eq!(
            surface_vertices.neighbors_of_edge(edges.z.swap_values()),
            [z1, z0, z3, z2],
        );
    }

    // TASK: Simplify test by using `SurfaceVertices` directly.
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

    fn test_vertices() -> BTreeMap<grid::Index, Point<f32, 3>> {
        let mut surface_vertices = BTreeMap::new();

        surface_vertices.insert([0, 0, 0].into(), [0.0, 0.0, 0.0].into());
        surface_vertices.insert([0, 0, 1].into(), [0.0, 0.0, 1.0].into());
        surface_vertices.insert([0, 1, 0].into(), [0.0, 1.0, 0.0].into());
        surface_vertices.insert([0, 1, 1].into(), [0.0, 1.0, 1.0].into());
        surface_vertices.insert([1, 0, 0].into(), [1.0, 0.0, 0.0].into());
        surface_vertices.insert([1, 0, 1].into(), [1.0, 0.0, 1.0].into());
        surface_vertices.insert([1, 1, 0].into(), [1.0, 1.0, 0.0].into());
        surface_vertices.insert([1, 1, 1].into(), [1.0, 1.0, 1.0].into());

        surface_vertices
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
