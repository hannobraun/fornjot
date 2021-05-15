use std::{collections::HashSet, ops::Range};

use itertools::Itertools as _;
use nalgebra::Point;
use num_traits::real::Real as _;
use tracing::{instrument, trace};

use crate::geometry::{aabb::Aabb, attributes::Distance, isosurface::Value};

use super::{Edge, GridIndex};

/// A grid for isosurface extraction
///
/// `min` and `max` define the minimum and maximum points of the isosurface.
/// `resolution` is the distance between points in the grid.
///
/// The actual values returned by `Grid`'s methods might be below or above that,
/// to enable proper extraction of the surface.
#[derive(Debug)]
pub struct GridDescriptor {
    pub aabb: Aabb<3>,
    pub resolution: f32,
}

impl GridDescriptor {
    /// Returns the grid points themselves
    ///
    /// The grid extends beyond the `min` and `max` values given to the
    /// constructor, so that the center of the outermost cubes are on the
    /// isosurface, or outside of it.
    pub fn points(
        &self,
    ) -> impl Iterator<Item = (GridIndex, Point<f32, 3>)> + '_ {
        let min = self.aabb.min;
        let max = self.aabb.max;

        let indices_x = indices(min.x, max.x, self.resolution);
        let indices_y = indices(min.y, max.y, self.resolution);
        let indices_z = indices(min.z, max.z, self.resolution);

        let indices = indices_x
            .cartesian_product(indices_y)
            .cartesian_product(indices_z)
            .map(|((x, y), z)| GridIndex::from([x, y, z]));

        let points = indices.map(move |index| {
            (index, index.to_coordinates(min, self.resolution))
        });

        points
    }

    #[instrument(skip(isosurface))]
    pub fn edges(&self, isosurface: &impl Distance<3>) -> HashSet<Edge> {
        let mut edges = HashSet::new();
        self.edges_inner(isosurface, self.aabb, &mut edges);
        edges
    }

    #[instrument(skip(self, isosurface))]
    fn edges_inner(
        &self,
        isosurface: &impl Distance<3>,
        aabb: Aabb<3>,
        edges: &mut HashSet<Edge>,
    ) {
        trace!("enter");

        let mut must_partition = false;

        for &[a, b] in &aabb.edges() {
            // TASK: Remove `index` from `Edge`, or use a more fitting struct.
            let edge = Edge {
                a: Value {
                    index: [0, 0, 0].into(),
                    point: [a.x.into(), a.y.into(), a.z.into()].into(),
                    value: isosurface.distance(a).into(),
                },
                b: Value {
                    index: [0, 0, 0].into(),
                    point: [b.x.into(), b.y.into(), b.z.into()].into(),
                    value: isosurface.distance(b).into(),
                },
            };

            let edge_length = edge.length();

            if edge.at_surface() {
                edges.insert(edge);
                continue;
            }
            if edge.a.value.abs() > edge_length
                && edge.b.value.abs() > edge_length
            {
                continue;
            }
            if edge_length > self.resolution {
                must_partition = true;
                continue;
            }
        }

        if must_partition {
            let [a, b, c, d, e, f, g, h] = aabb.partition();

            self.edges_inner(isosurface, a, edges);
            self.edges_inner(isosurface, b, edges);
            self.edges_inner(isosurface, c, edges);
            self.edges_inner(isosurface, d, edges);
            self.edges_inner(isosurface, e, edges);
            self.edges_inner(isosurface, f, edges);
            self.edges_inner(isosurface, g, edges);
            self.edges_inner(isosurface, h, edges);
        }
    }
}

fn indices(min: f32, max: f32, resolution: f32) -> Range<usize> {
    let lower = 0;
    let upper = ((max - min) / resolution).ceil() as usize + 2;

    lower..upper
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use decorum::R32;

    use crate::geometry::{aabb::Aabb, shapes::Sphere};

    use super::GridDescriptor;

    #[test]
    fn points_should_return_grid_points() {
        let grid = GridDescriptor {
            aabb: Aabb {
                min: [0.0, 0.0, 0.5].into(),
                max: [0.5, 1.0, 1.5].into(),
            },
            resolution: 1.0,
        };

        let points: Vec<_> = grid.points().collect();

        assert_eq!(
            points,
            vec![
                ([0, 0, 0].into(), [-0.5, -0.5, 0.0].into()),
                ([0, 0, 1].into(), [-0.5, -0.5, 1.0].into()),
                ([0, 0, 2].into(), [-0.5, -0.5, 2.0].into()),
                ([0, 1, 0].into(), [-0.5, 0.5, 0.0].into()),
                ([0, 1, 1].into(), [-0.5, 0.5, 1.0].into()),
                ([0, 1, 2].into(), [-0.5, 0.5, 2.0].into()),
                ([0, 2, 0].into(), [-0.5, 1.5, 0.0].into()),
                ([0, 2, 1].into(), [-0.5, 1.5, 1.0].into()),
                ([0, 2, 2].into(), [-0.5, 1.5, 2.0].into()),
                ([1, 0, 0].into(), [0.5, -0.5, 0.0].into()),
                ([1, 0, 1].into(), [0.5, -0.5, 1.0].into()),
                ([1, 0, 2].into(), [0.5, -0.5, 2.0].into()),
                ([1, 1, 0].into(), [0.5, 0.5, 0.0].into()),
                ([1, 1, 1].into(), [0.5, 0.5, 1.0].into()),
                ([1, 1, 2].into(), [0.5, 0.5, 2.0].into()),
                ([1, 2, 0].into(), [0.5, 1.5, 0.0].into()),
                ([1, 2, 1].into(), [0.5, 1.5, 1.0].into()),
                ([1, 2, 2].into(), [0.5, 1.5, 2.0].into()),
                ([2, 0, 0].into(), [1.5, -0.5, 0.0].into()),
                ([2, 0, 1].into(), [1.5, -0.5, 1.0].into()),
                ([2, 0, 2].into(), [1.5, -0.5, 2.0].into()),
                ([2, 1, 0].into(), [1.5, 0.5, 0.0].into()),
                ([2, 1, 1].into(), [1.5, 0.5, 1.0].into()),
                ([2, 1, 2].into(), [1.5, 0.5, 2.0].into()),
                ([2, 2, 0].into(), [1.5, 1.5, 0.0].into()),
                ([2, 2, 1].into(), [1.5, 1.5, 1.0].into()),
                ([2, 2, 2].into(), [1.5, 1.5, 2.0].into()),
            ]
        );
    }

    #[test]
    fn edges_should_return_edges_at_surface() {
        let grid = GridDescriptor {
            aabb: Aabb {
                min: [-1.0, -1.0, -1.0].into(),
                max: [1.0, 1.0, 1.0].into(),
            },
            resolution: 1.0,
        };

        let edges: HashSet<_> = grid
            .edges(&Sphere::new().with_radius(0.5))
            .into_iter()
            .map(|edge| {
                let a = edge.a.point;
                let b = edge.b.point;

                let a: [R32; 3] = [a.x.into(), a.y.into(), a.z.into()];
                let b: [R32; 3] = [b.x.into(), b.y.into(), b.z.into()];

                [a, b]
            })
            .collect();

        #[rustfmt::skip]
        let expected_edges = [
            [[ 0.0,  0.0, -1.0], [0.0, 0.0, 0.0]],
            [[ 0.0, -1.0,  0.0], [0.0, 0.0, 0.0]],
            [[-1.0,  0.0,  0.0], [0.0, 0.0, 0.0]],
            [[ 0.0,  0.0,  0.0], [0.0, 0.0, 1.0]],
            [[ 0.0,  0.0,  0.0], [0.0, 1.0, 0.0]],
            [[ 0.0,  0.0,  0.0], [1.0, 0.0, 0.0]],
        ];

        assert_eq!(edges.len(), expected_edges.len());
        for &[[ax, ay, az], [bx, by, bz]] in &expected_edges {
            let edge = [
                [ax.into(), ay.into(), az.into()],
                [bx.into(), by.into(), bz.into()],
            ];
            assert!(edges.contains(&edge));
        }
    }
}
