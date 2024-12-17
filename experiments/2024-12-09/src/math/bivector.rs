use super::Vector;

pub struct Bivector<const D: usize> {
    pub a: Vector<D>,
    pub b: Vector<D>,
}
