pub struct Difference<A, B> {
    pub a: A,
    pub b: B,
}

pub trait MakeDifference<A, B> {
    fn difference(self) -> Difference<A, B>;
}

impl<A, B> MakeDifference<A, B> for (A, B) {
    fn difference(self) -> Difference<A, B> {
        Difference {
            a: self.0,
            b: self.1,
        }
    }
}
