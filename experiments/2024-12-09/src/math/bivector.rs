use super::Vector;

#[derive(Clone, Copy)]
pub struct Bivector<const D: usize> {
    pub a: Vector<D>,
    pub b: Vector<D>,
}
