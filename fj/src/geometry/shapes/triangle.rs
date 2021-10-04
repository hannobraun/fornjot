use super::Polygon;

/// A triangle, defined in terms of a 3-sided `Polygon`
pub type Triangle<const D: usize> = Polygon<D, 3>;
