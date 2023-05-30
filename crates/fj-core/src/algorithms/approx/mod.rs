//! Approximation of objects

pub mod cycle;
pub mod edge;
pub mod face;
pub mod path;
pub mod shell;
pub mod sketch;
pub mod solid;
pub mod tolerance;

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use fj_math::Point;

pub use self::tolerance::{InvalidTolerance, Tolerance};

/// Approximate an object
pub trait Approx: Sized {
    /// The approximation of the object
    type Approximation;

    /// The cache used to cache approximation results
    type Cache: Default;

    /// Approximate the object
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual object.
    fn approx(self, tolerance: impl Into<Tolerance>) -> Self::Approximation {
        let mut cache = Self::Cache::default();
        self.approx_with_cache(tolerance, &mut cache)
    }

    /// Approximate the object, using the provided cache
    ///
    /// This is a lower-level method that allows some degree of control over
    /// caching. Callers might consider using [`Approx::approx`] instead.
    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation;
}

/// A point from an approximation, with local and global forms
#[derive(Debug, Clone)]
pub struct ApproxPoint<const D: usize> {
    /// The local form of the point
    pub local_form: Point<D>,

    /// The global form of the points
    pub global_form: Point<3>,
}

impl<const D: usize> ApproxPoint<D> {
    /// Create an instance of `ApproxPoint`, without a source
    pub fn new(local_form: Point<D>, global_form: Point<3>) -> Self {
        Self {
            local_form,
            global_form,
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
