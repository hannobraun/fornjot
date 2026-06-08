use super::{Point, Vector};

/// An axis-aligned bounding box (AABB)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Aabb<const D: usize> {
    /// The minimum coordinates of the AABB
    pub min: Point<D>,

    /// The maximum coordinates of the AABB
    pub max: Point<D>,
}

impl<const D: usize> Aabb<D> {
    /// # Construct an axis-aligned bounding box from a list of points
    ///
    /// Returns an axis-aligned-bounding box that contains all the points from
    /// the provided iterator. The iterator must yield at least one point.
    ///
    /// ## Panics
    ///
    /// Panics, if the provided iterator is empty.
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<D>>>,
    ) -> Self {
        let mut points = points.into_iter().map(Into::into);

        let Some(initial_point) = points.next() else {
            panic!("Must provide at least one point to `Aabb::from_points`.");
        };

        let mut min = initial_point;
        let mut max = initial_point;

        for point in points {
            min = Point::min(min, point);
            max = Point::max(max, point);
        }

        Self { min, max }
    }

    /// Compute the center point of the AABB
    pub fn center(&self) -> Point<D> {
        self.min + self.size() / 2.
    }

    /// Compute the size of the AABB
    pub fn size(&self) -> Vector<D> {
        self.max - self.min
    }

    /// Determine whether the AABB contains a given point
    pub fn contains(&self, point: impl Into<Point<D>>) -> bool {
        let point = point.into();

        let min = self
            .min
            .coords
            .components
            .into_iter()
            .zip(point.coords.components);
        for (min, p) in min {
            if min > p {
                return false;
            }
        }

        let max = self
            .max
            .coords
            .components
            .into_iter()
            .zip(point.coords.components);
        for (max, p) in max {
            if max < p {
                return false;
            }
        }

        true
    }

    /// Merge this AABB with another
    pub fn merged(&self, other: &Self) -> Self {
        Self::from_points([self.min, self.max, other.min, other.max])
    }
}

impl Aabb<3> {
    /// Access the vertices of the AABB
    pub fn vertices(&self) -> [Point<3>; 8] {
        let [min_x, min_y, min_z] = self.min.coords.components;
        let [max_x, max_y, max_z] = self.min.coords.components;

        [
            [min_x, min_y, min_z],
            [min_x, min_y, max_z],
            [min_x, max_y, min_z],
            [min_x, max_y, max_z],
            [max_x, min_y, min_z],
            [max_x, min_y, max_z],
            [max_x, max_y, min_z],
            [max_x, max_y, max_z],
        ]
        .map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Point;

    use super::Aabb;

    #[test]
    fn from_points() {
        assert_eq!(
            Aabb::from_points([[1., 2.], [2., 1.]]),
            Aabb {
                min: Point::from([1., 1.]),
                max: Point::from([2., 2.]),
            },
        );
    }

    #[test]
    fn contains() {
        let aabb = Aabb::<2>::from_points([[1., 1.], [3., 3.]]);

        assert!(aabb.contains([2., 2.]));

        assert!(!aabb.contains([0., 0.]));
        assert!(!aabb.contains([4., 0.]));
        assert!(!aabb.contains([4., 4.]));
        assert!(!aabb.contains([0., 4.]));

        assert!(!aabb.contains([2., 0.]));
        assert!(!aabb.contains([2., 4.]));
        assert!(!aabb.contains([0., 2.]));
        assert!(!aabb.contains([4., 2.]));
    }
}
