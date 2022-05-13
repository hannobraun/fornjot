use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::validate::Validate;

/// Marker trait for geometric and topological objects
pub trait Object:
    'static + Clone + PartialEq + Validate + private::Sealed
{
}

impl private::Sealed for Point<3> {}
impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge {}
impl private::Sealed for Cycle {}
impl private::Sealed for Face {}

impl Object for Point<3> {}
impl Object for Curve<3> {}
impl Object for Surface {}

impl Object for Vertex {}
impl Object for Edge {}
impl Object for Cycle {}
impl Object for Face {}

mod private {
    pub trait Sealed {}
}
