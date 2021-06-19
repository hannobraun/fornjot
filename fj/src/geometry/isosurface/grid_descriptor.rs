use std::ops::Range;

use itertools::Itertools as _;
use nalgebra::Point;

use crate::geometry::aabb::Aabb;

use super::GridIndex;

/// A grid for isosurface extraction
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
}

fn indices(min: f32, max: f32, resolution: f32) -> Range<usize> {
    let lower = 0;
    let upper = ((max - min) / resolution).ceil() as usize + 2;

    lower..upper
}

#[cfg(test)]
mod tests {
    use crate::geometry::aabb::Aabb;

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
}
