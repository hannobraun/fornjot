use std::fmt;

use crate::{geometry::operations::Translate, math::Point};

/// A 0-dimensional vertex
///
/// By itself, this type is not very useful. You can apply meaning to it by
/// using various operations on it, for example by sweeping it to create a line,
/// or by giving it a position in space using a translation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex;

impl<const D: usize> Translate<Vertex, D> {
    pub fn display(&self) -> impl fmt::Display {
        self.offset
    }
}

impl<const D: usize> From<Point<D>> for Translate<Vertex, D> {
    fn from(point: Point<D>) -> Self {
        Translate {
            shape: Vertex,
            offset: point.coords,
        }
    }
}
