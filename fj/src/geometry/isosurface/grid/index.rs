use std::ops::Add;

use nalgebra::Point;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Index([usize; 3]);

impl Index {
    pub fn x(&self) -> usize {
        self.0[0]
    }

    pub fn y(&self) -> usize {
        self.0[1]
    }

    pub fn z(&self) -> usize {
        self.0[2]
    }

    pub fn to_coordinates(
        &self,
        min: Point<f32, 3>,
        resolution: f32,
    ) -> Point<f32, 3> {
        [
            index_to_coordinate(self.x(), min.x, resolution),
            index_to_coordinate(self.y(), min.y, resolution),
            index_to_coordinate(self.z(), min.z, resolution),
        ]
        .into()
    }
}

impl From<[usize; 3]> for Index {
    fn from(index: [usize; 3]) -> Self {
        Self(index)
    }
}

impl Add<[usize; 3]> for Index {
    type Output = Self;

    fn add(mut self, rhs: [usize; 3]) -> Self::Output {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];

        self
    }
}

fn index_to_coordinate(index: usize, min: f32, resolution: f32) -> f32 {
    index as f32 * resolution + min - resolution / 2.0
}
