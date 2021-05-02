use std::{iter, ops::Range};

use itertools::Itertools as _;
use nalgebra::Point;

/// A grid for isosurface extraction
///
/// `min` and `max` define the minimum and maximum points of the isosurface.
/// `resolution` is the distance between points in the grid.
///
/// The actual values returned by `Grid`'s methods might be below or above that,
/// to enable proper extraction of the surface.
pub struct Grid {
    pub min: Point<f32, 3>,
    pub max: Point<f32, 3>,
    pub resolution: f32,
}

impl Grid {
    /// Returns the grid points themselves
    ///
    /// The grid extends beyond the `min` and `max` values given to the
    /// constructor, so that the center of the outermost cubes are on the
    /// isosurface, or outside of it.
    pub fn points(
        &self,
    ) -> impl Iterator<Item = (Point<usize, 3>, Point<f32, 3>)> + '_ {
        let indices_x = grid_indices(self.min.x, self.max.x, self.resolution);
        let indices_y = grid_indices(self.min.y, self.max.y, self.resolution);
        let indices_z = grid_indices(self.min.z, self.max.z, self.resolution);

        let indices = indices_x
            .cartesian_product(indices_y)
            .cartesian_product(indices_z)
            .map(|((x, y), z)| [x, y, z]);

        let points = indices
            .map(move |[x, y, z]| {
                (
                    [x, y, z],
                    [
                        grid_index_to_coordinate(
                            x,
                            self.min.x,
                            self.resolution,
                        ),
                        grid_index_to_coordinate(
                            y,
                            self.min.y,
                            self.resolution,
                        ),
                        grid_index_to_coordinate(
                            z,
                            self.min.z,
                            self.resolution,
                        ),
                    ],
                )
            })
            .map(|(index, point)| (index.into(), point.into()));

        points
    }

    /// Returns the centers of all grid cubes
    ///
    /// The grid is made up of points at regular intervals that form cubes. This
    /// method returns an iterator over the center of these cubes.
    pub fn cube_centers(
        &self,
    ) -> impl Iterator<Item = (Point<usize, 3>, Point<f32, 3>)> {
        // TASK: Implement
        iter::empty()
    }
}

fn grid_indices(min: f32, max: f32, resolution: f32) -> Range<usize> {
    let lower = 0;
    let upper = ((max - min) / resolution).ceil() as usize + 2;

    lower..upper
}

fn grid_index_to_coordinate(index: usize, min: f32, resolution: f32) -> f32 {
    index as f32 * resolution + min - resolution / 2.0
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn grid_points_should_return_grid_points() {
        let grid = Grid {
            min: [0.0, 0.0, 0.5].into(),
            max: [0.5, 1.0, 1.5].into(),
            resolution: 1.0,
        };

        let grid_points: Vec<_> = grid.points().collect();

        assert_eq!(
            grid_points,
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
