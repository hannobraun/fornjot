use parry3d_f64::bounding_volume::BoundingVolume as _;

use super::{Point, Vector};

/// An axis-aligned bounding box (AABB)
pub struct Aabb {
    /// The minimum coordinates of the AABB
    pub mins: Point<3>,

    /// The maximum coordinates of the AABB
    pub maxs: Point<3>,
}

impl Aabb {
    /// Construct an AABB from a list of points
    ///
    /// The resulting AABB will contain all the points.
    pub fn from_points(points: impl IntoIterator<Item = Point<3>>) -> Self {
        let points: Vec<_> = points.into_iter().collect();
        parry3d_f64::bounding_volume::AABB::from_points(&points).into()
    }

    /// Construct an AABB from a Parry AABB
    pub fn from_parry(aabb: parry3d_f64::bounding_volume::AABB) -> Self {
        Self {
            mins: aabb.mins.into(),
            maxs: aabb.maxs.into(),
        }
    }

    /// Convert the AABB to a Parry AABB
    pub fn to_parry(&self) -> parry3d_f64::bounding_volume::AABB {
        parry3d_f64::bounding_volume::AABB {
            mins: self.mins,
            maxs: self.maxs,
        }
    }

    /// Access the vertices of the AABB
    pub fn vertices(&self) -> [Point<3>; 8] {
        self.to_parry().vertices()
    }

    /// Compute the center point of the AABB
    pub fn center(&self) -> Point<3> {
        self.to_parry().center()
    }

    /// Compute the size of the AABB
    pub fn extents(&self) -> Vector<3> {
        self.to_parry().extents().into()
    }

    /// Merge this AABB with another
    pub fn merged(&self, other: &Aabb) -> Aabb {
        self.to_parry().merged(&other.to_parry()).into()
    }
}

impl From<parry3d_f64::bounding_volume::AABB> for Aabb {
    fn from(aabb: parry3d_f64::bounding_volume::AABB) -> Self {
        Self::from_parry(aabb)
    }
}
