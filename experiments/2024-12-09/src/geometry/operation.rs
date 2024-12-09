use std::fmt;

use super::{Triangle, Vertex};

pub trait Operation: fmt::Display {
    fn vertices(&self, vertices: &mut Vec<Vertex>);
    fn triangles(&self, triangles: &mut Vec<Triangle>);
    fn children(&self) -> Vec<Box<dyn Operation>>;
}
