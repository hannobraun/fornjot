use super::Polygon;

/// A quadrilateral, defined in terms of a 4-sided `Polygon`
pub type Quad<const D: usize> = Polygon<D, 4>;
