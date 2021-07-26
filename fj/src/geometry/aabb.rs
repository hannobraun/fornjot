use std::fmt;

use nalgebra::{Point, SVector};

use crate::geometry::util::DebugPoint;

/// Axis-aligned bounding box
#[derive(Clone, Copy, PartialEq)]
pub struct Aabb<const D: usize> {
    /// Minimum point of the axis-aligned bounding box
    pub min: Point<f32, D>,

    /// Maximum point of the axis-aligned bounding box
    pub max: Point<f32, D>,
}

impl<const D: usize> Aabb<D> {
    /// Construct `Aabb` from minimum point and size
    ///
    /// # Panics
    /// Panics, if `size` has at least one negative component, or a magnitude of
    /// zero.
    pub fn from_min_and_size(
        min: Point<f32, D>,
        size: SVector<f32, D>,
    ) -> Self {
        assert!(size[0] >= 0.0 && size[1] >= 0.0 && size[2] >= 0.0);
        assert!(size.magnitude_squared() > 0.0);

        Self {
            min,
            max: min + size,
        }
    }

    /// Size of the axis-aligned bounding box
    pub fn size(&self) -> SVector<f32, D> {
        self.max - self.min
    }

    /// Center point of the axis-aligned bounding box
    pub fn center(&self) -> Point<f32, D> {
        self.min + self.size() / 2.0
    }
}

impl Aabb<2> {
    /// Extend 2-dimensional `Aabb` with a third dimension
    pub fn extend(self, min: f32, max: f32) -> Aabb<3> {
        Aabb {
            min: [self.min.x, self.min.y, min].into(),
            max: [self.max.x, self.max.y, max].into(),
        }
    }
}

impl Aabb<3> {
    /// Vertices of the axis-aligned bounding box
    pub fn vertices(&self) -> [Point<f32, 3>; 8] {
        [
            [self.min.x, self.min.y, self.min.z].into(),
            [self.min.x, self.min.y, self.max.z].into(),
            [self.min.x, self.max.y, self.min.z].into(),
            [self.min.x, self.max.y, self.max.z].into(),
            [self.max.x, self.min.y, self.min.z].into(),
            [self.max.x, self.min.y, self.max.z].into(),
            [self.max.x, self.max.y, self.min.z].into(),
            [self.max.x, self.max.y, self.max.z].into(),
        ]
    }

    /// Edges of the axis-aligned bounding
    pub fn edges(&self) -> [[Point<f32, 3>; 2]; 12] {
        let [a, b, c, d, e, f, g, h] = self.vertices();

        [
            [a, b],
            [a, c],
            [a, e],
            [b, d],
            [b, f],
            [c, d],
            [c, g],
            [d, h],
            [e, f],
            [e, g],
            [f, h],
            [g, h],
        ]
    }
}

impl<const D: usize> fmt::Debug for Aabb<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}, {:?}]",
            DebugPoint(self.min),
            DebugPoint(self.max)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;

    #[test]
    fn center_should_return_center() {
        let aabb = Aabb {
            min: [1.0, 2.0].into(),
            max: [3.0, 4.0].into(),
        };

        assert_eq!(aabb.center(), [2.0, 3.0].into());
    }

    #[test]
    fn vertices_should_return_vertices() {
        let aabb = Aabb {
            min: [0.0, 0.0, 0.0].into(),
            max: [1.0, 1.0, 1.0].into(),
        };

        assert_eq!(
            aabb.vertices(),
            [
                [0.0, 0.0, 0.0].into(),
                [0.0, 0.0, 1.0].into(),
                [0.0, 1.0, 0.0].into(),
                [0.0, 1.0, 1.0].into(),
                [1.0, 0.0, 0.0].into(),
                [1.0, 0.0, 1.0].into(),
                [1.0, 1.0, 0.0].into(),
                [1.0, 1.0, 1.0].into(),
            ]
        )
    }

    #[test]
    fn edges_should_return_edges() {
        let aabb = Aabb {
            min: [0.0, 0.0, 0.0].into(),
            max: [1.0, 1.0, 1.0].into(),
        };

        assert_eq!(
            aabb.edges(),
            [
                [[0.0, 0.0, 0.0].into(), [0.0, 0.0, 1.0].into(),],
                [[0.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into(),],
                [[0.0, 0.0, 0.0].into(), [1.0, 0.0, 0.0].into(),],
                [[0.0, 0.0, 1.0].into(), [0.0, 1.0, 1.0].into(),],
                [[0.0, 0.0, 1.0].into(), [1.0, 0.0, 1.0].into(),],
                [[0.0, 1.0, 0.0].into(), [0.0, 1.0, 1.0].into(),],
                [[0.0, 1.0, 0.0].into(), [1.0, 1.0, 0.0].into(),],
                [[0.0, 1.0, 1.0].into(), [1.0, 1.0, 1.0].into(),],
                [[1.0, 0.0, 0.0].into(), [1.0, 0.0, 1.0].into(),],
                [[1.0, 0.0, 0.0].into(), [1.0, 1.0, 0.0].into(),],
                [[1.0, 0.0, 1.0].into(), [1.0, 1.0, 1.0].into(),],
                [[1.0, 1.0, 0.0].into(), [1.0, 1.0, 1.0].into(),],
            ]
        );
    }
}
