//! Approximation of objects

pub mod curve;
pub mod cycle;
pub mod edge;
pub mod face;
pub mod shell;
pub mod sketch;
pub mod solid;
pub mod tolerance;

use std::{
    any::Any,
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::Rc,
};

use fj_math::Point;

use crate::objects::Curve;

pub use self::tolerance::{InvalidTolerance, Tolerance};

/// Approximate an object
pub trait Approx {
    /// The approximation of the object
    type Approximation;

    /// Approximate the object
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual object.
    fn approx(self, tolerance: Tolerance) -> Self::Approximation;
}

/// A point from an approximation, with local and global forms
#[derive(Debug, Clone)]
pub struct ApproxPoint<const D: usize> {
    /// The local form of the point
    pub local_form: Point<D>,

    /// The global form of the points
    pub global_form: Point<3>,

    /// The optional source of the point
    pub source: Option<Rc<dyn Source>>,
}

impl<const D: usize> ApproxPoint<D> {
    /// Create an instance of `ApproxPoint`, without a source
    pub fn new(local_form: Point<D>, global_form: Point<3>) -> Self {
        Self {
            local_form,
            global_form,
            source: None,
        }
    }

    /// Attach a source to the point
    pub fn with_source(self, source: impl Source) -> Self {
        Self {
            source: Some(Rc::new(source)),
            ..self
        }
    }
}

impl<const D: usize> Eq for ApproxPoint<D> {}

impl<const D: usize> PartialEq for ApproxPoint<D> {
    fn eq(&self, other: &Self) -> bool {
        self.local_form == other.local_form
            && self.global_form == other.global_form
    }
}

impl<const D: usize> Hash for ApproxPoint<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.local_form.hash(state);
        self.global_form.hash(state);
    }
}

impl<const D: usize> Ord for ApproxPoint<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.local_form.cmp(&other.local_form) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.global_form.cmp(&other.global_form)
    }
}

impl<const D: usize> PartialOrd for ApproxPoint<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The source of an [`ApproxPoint`]
pub trait Source: Any + Debug {}

impl Source for (Curve, Point<1>) {}
