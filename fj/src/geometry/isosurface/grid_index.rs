#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GridIndex(pub [usize; 3]);

impl GridIndex {
    pub fn x(&self) -> usize {
        self.0[0]
    }

    pub fn y(&self) -> usize {
        self.0[1]
    }

    pub fn z(&self) -> usize {
        self.0[2]
    }
}

impl From<[usize; 3]> for GridIndex {
    fn from(index: [usize; 3]) -> Self {
        Self(index)
    }
}
