use std::cmp;

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
    /// Construct an AABB from a list of points
    ///
    /// Returns an axis-aligned-bounding box that contains all the points from
    /// the provided iterator, if the iterator yields any points. Returns `None`
    /// otherwise.
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<D>>>,
    ) -> Option<Self> {
        let mut points = points.into_iter().map(Into::into);

        let initial_point = points.next()?;

        let mut aabb = Self {
            min: initial_point,
            max: initial_point,
        };

        for point in points {
            aabb.min
                .coords
                .components
                .iter_mut()
                .zip(point.coords.components.iter())
                .for_each(|(min, &new)| *min = cmp::min(*min, new));

            aabb.max
                .coords
                .components
                .iter_mut()
                .zip(point.coords.components.iter())
                .for_each(|(max, &new)| *max = cmp::max(*max, new));
        }

        Some(aabb)
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
            .expect("Provided points, so must get an AABB back.")
    }
}

impl Aabb<3> {
    /// Convert the AABB to a Parry AABB
    pub fn to_parry(self) -> parry3d_f64::bounding_volume::Aabb {
        parry3d_f64::bounding_volume::Aabb {
            mins: self.min.to_na(),
            maxs: self.max.to_na(),
        }
    }

    /// Access the vertices of the AABB
    pub fn vertices(&self) -> [Point<3>; 8] {
        self.to_parry().vertices().map(Into::into)
    }

    /// Compute the center point of the AABB
    pub fn center(&self) -> Point<3> {
        self.min + self.size() / 2.
    }

    /// Compute the size of the AABB
    pub fn size(&self) -> Vector<3> {
        self.max - self.min
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;

    #[test]
    fn contains() {
        let Some(aabb) = Aabb::<2>::from_points([[1., 1.], [3., 3.]]) else {
            unreachable!(
                "We provided points, so the constructor must return an AABB."
            );
        };

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
