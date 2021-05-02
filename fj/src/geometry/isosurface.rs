use std::iter;

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
    pub fn grid_points(&self) -> impl Iterator<Item = Point<f32, 3>> {
        // TASK: Implement
        iter::empty()
    }

    /// Returns the centers of all grid cubes
    ///
    /// The grid is made up of points at regular intervals that form cubes. This
    /// method returns an iterator over the center of these cubes.
    pub fn cube_centers(&self) -> impl Iterator<Item = Point<f32, 3>> {
        // TASK: Implement
        iter::empty()
    }
}
