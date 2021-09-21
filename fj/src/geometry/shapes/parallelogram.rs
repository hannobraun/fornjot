use crate::{
    geometry::{operations::Sweep, shapes::Line},
    math::Vector,
};

pub type Parallelogram = Sweep<Line, Vector<2>>;
