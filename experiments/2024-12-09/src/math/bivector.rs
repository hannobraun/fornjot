use super::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Bivector<const D: usize> {
    pub a: Vector<D>,
    pub b: Vector<D>,
}
