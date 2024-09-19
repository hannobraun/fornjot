//! Approximation of objects

pub mod cycle;
pub mod face;
pub mod half_edge;
pub mod shell;
pub mod sketch;
pub mod solid;

mod circle;
mod curve;
mod line;
mod vertex;

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use curve::CurveApproxCache;
use fj_math::Point;
use vertex::VertexApproxCache;

use crate::geometry::{Geometry, Tolerance};

pub use self::circle::CircleApproxParams;

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
    fn approx(
        self,
        tolerance: impl Into<Tolerance>,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let mut cache = Self::Cache::default();
        self.approx_with_cache(tolerance, &mut cache, geometry)
    }

    /// Approximate the object, using the provided cache
    ///
    /// This is a lower-level method that allows some degree of control over
    /// caching. Callers might consider using [`Approx::approx`] instead.
    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation;
}

/// Cache for half-edge approximations
#[derive(Default)]
pub struct ApproxCache {
    /// Cache for vertex approximations
    pub vertex: VertexApproxCache,

    /// Cache for curve approximations
    pub curve: CurveApproxCache,
}

/// A point from an approximation, with local and global forms
#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint<const D: usize> {
    /// The local form of the point
    pub local_form: Point<D>,

    /// The global form of the points
    pub global_form: Point<3>,
}

impl<const D: usize> ApproxPoint<D> {
    /// Create an instance of `ApproxPoint`
    pub fn new(
        local_form: impl Into<Point<D>>,
        global_form: impl Into<Point<3>>,
    ) -> Self {
        Self {
            local_form: local_form.into(),
            global_form: global_form.into(),
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
