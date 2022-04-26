use parry3d_f64::bounding_volume::BoundingVolume as _;

use super::{Point, Vector};

/// An axis-aligned bounding box (AABB)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Aabb<const D: usize> {
    /// The minimum coordinates of the AABB
    pub min: Point<D>,

    /// The maximum coordinates of the AABB
    pub max: Point<D>,
}

impl Aabb<2> {
    /// Construct a 2-dimensional AABB from a list of points
    ///
    /// The resulting AABB will contain all the points.
    pub fn from_points(points: impl IntoIterator<Item = Point<2>>) -> Self {
        let points: Vec<_> =
            points.into_iter().map(|point| point.to_na()).collect();
        parry2d_f64::bounding_volume::AABB::from_points(&points).into()
    }

    /// Construct a 2-dimensional AABB from a Parry AABB
    pub fn from_parry(aabb: parry2d_f64::bounding_volume::AABB) -> Self {
        Self {
            min: aabb.mins.into(),
            max: aabb.maxs.into(),
        }
    }
}

impl Aabb<3> {
    /// Construct a 3-dimensional AABB from a list of points
    ///
    /// The resulting AABB will contain all the points.
    pub fn from_points(points: impl IntoIterator<Item = Point<3>>) -> Self {
        let points: Vec<_> =
            points.into_iter().map(|point| point.to_na()).collect();
        parry3d_f64::bounding_volume::AABB::from_points(&points).into()
    }

    /// Construct a 3-dimensional AABB from a Parry AABB
    pub fn from_parry(aabb: parry3d_f64::bounding_volume::AABB) -> Self {
        Self {
            min: aabb.mins.into(),
            max: aabb.maxs.into(),
        }
    }

    /// Convert the AABB to a Parry AABB
    pub fn to_parry(self) -> parry3d_f64::bounding_volume::AABB {
        parry3d_f64::bounding_volume::AABB {
            mins: self.min.to_na(),
            maxs: self.max.to_na(),
        }
    }

    /// Access the vertices of the AABB
    pub fn vertices(&self) -> [Point<3>; 8] {
        self.to_parry().vertices().map(|vertex| vertex.into())
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

impl From<parry2d_f64::bounding_volume::AABB> for Aabb<2> {
    fn from(aabb: parry2d_f64::bounding_volume::AABB) -> Self {
        Self::from_parry(aabb)
    }
}

impl From<parry3d_f64::bounding_volume::AABB> for Aabb<3> {
    fn from(aabb: parry3d_f64::bounding_volume::AABB) -> Self {
        Self::from_parry(aabb)
    }
}
