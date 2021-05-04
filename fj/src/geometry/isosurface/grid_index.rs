#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GridIndex(pub [usize; 3]);

impl From<[usize; 3]> for GridIndex {
    fn from(index: [usize; 3]) -> Self {
        Self(index)
    }
}
