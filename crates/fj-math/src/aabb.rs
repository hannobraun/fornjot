use parry2d_f64::bounding_volume::BoundingVolume as _;
use parry3d_f64::bounding_volume::BoundingVolume as _;

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
}

impl Aabb<2> {
    /// Construct a 2-dimensional AABB from a list of points
    ///
    /// The resulting AABB will contain all the points.
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = points.into_iter().map(|point| point.into().to_na());
        parry2d_f64::bounding_volume::Aabb::from_points(points).into()
    }

    /// Construct a 2-dimensional AABB from a Parry AABB
    pub fn from_parry(aabb: parry2d_f64::bounding_volume::Aabb) -> Self {
        Self {
            min: aabb.mins.into(),
            max: aabb.maxs.into(),
        }
    }

    /// Convert the AABB to a Parry AABB
    pub fn to_parry(self) -> parry2d_f64::bounding_volume::Aabb {
        parry2d_f64::bounding_volume::Aabb {
            mins: self.min.to_na(),
            maxs: self.max.to_na(),
        }
    }

    /// Merge this AABB with another
    pub fn merged(&self, other: &Self) -> Self {
        self.to_parry().merged(&other.to_parry()).into()
    }
}

impl Aabb<3> {
    /// Construct a 3-dimensional AABB from a list of points
    ///
    /// The resulting AABB will contain all the points.
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<3>>>,
    ) -> Self {
        let points: Vec<_> = points
            .into_iter()
            .map(|point| point.into().to_na())
            .collect();
        parry3d_f64::bounding_volume::Aabb::from_points(points).into()
    }

    /// Construct a 3-dimensional AABB from a Parry AABB
    pub fn from_parry(aabb: parry3d_f64::bounding_volume::Aabb) -> Self {
        Self {
            min: aabb.mins.into(),
            max: aabb.maxs.into(),
        }
    }

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
        self.to_parry().center().into()
    }

    /// Compute the size of the AABB
    pub fn size(&self) -> Vector<3> {
        self.to_parry().extents().into()
    }

    /// Compute an AABB that includes an additional point
    pub fn include_point(self, point: &Point<3>) -> Self {
        let mut aabb = self.to_parry();
        aabb.take_point(point.to_na());

        Self::from_parry(aabb)
    }

    /// Merge this AABB with another
    pub fn merged(&self, other: &Self) -> Self {
        self.to_parry().merged(&other.to_parry()).into()
    }
}

impl From<parry2d_f64::bounding_volume::Aabb> for Aabb<2> {
    fn from(aabb: parry2d_f64::bounding_volume::Aabb) -> Self {
        Self::from_parry(aabb)
    }
}

impl From<parry3d_f64::bounding_volume::Aabb> for Aabb<3> {
    fn from(aabb: parry3d_f64::bounding_volume::Aabb) -> Self {
        Self::from_parry(aabb)
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;

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
