//! # Tools for creating approximations
//!
//! Fornjot can be viewed as a hybrid b-rep/mesh-based kernel. Topology is
//! represented using typical b-rep primitives, but geometry is approximated
//! with polylines and triangle meshes.
//!
//! Topological primitives and geometrical approximations exist side by side and
//! approximations are built up together with the topological primitives. The
//! tools provided by this module help doing that.
//!
//! This module is intended for internal use, as well as more advanced users of
//! Fornjot. It is typically required to implement operations that create and
//! modify shapes. More basic users would just use operations that others have
//! implemented, never coming into contact with this module.

use fj_math::Scalar;

mod half_edge;
mod point;

pub use self::{half_edge::ApproxHalfEdge, point::ApproxPoint};

/// # Provides iterators over the coordinates of an axis
///
/// See [`ApproxHalfEdge::from_start_and_axes`].
pub enum ApproxAxis {
    /// # Provide one fixed coordinate for the whole axis
    Fixed {
        /// # The fixed coordinate value
        value: Scalar,
    },

    /// # Provide uniformly distributed coordinates between `0` and `1`
    ///
    /// The number of coordinates provided is determined by the argument passed
    /// when calling [`ApproxAxis::into_iter`]. The coordinates provided will be
    /// _between_ `0` and `1`, excluding those limits.
    Uniform {
        /// # Indicate whether to reverse the coordinates
        ///
        /// Start with the lowest coordinate (the one closest to `0`), if this
        /// is false. Start with the highest coordinate (the one closest to
        /// `1`), if this is true.
        reverse: bool,
    },
}

impl ApproxAxis {
    /// # Convenience constructor to create an [`ApproxAxis::Fixed`]
    ///
    /// Allows you to supply any value that can convert into a [`Scalar`], while
    /// constructing [`ApproxAxis::Fixed`] directly requires you to provide a
    /// [`Scalar`] itself.
    pub fn fixed(value: impl Into<Scalar>) -> Self {
        let value = value.into();
        Self::Fixed { value }
    }

    /// # Iterate over the coordinates of this axis
    ///
    /// The returned iterator will yield the number of coordinates defined by
    /// the `num_coords` parameter. The values of those coordinates depend on
    /// the variant of this `ApproxAxis` instance.
    pub fn into_iter(self, num_coords: usize) -> impl Iterator<Item = Scalar> {
        match self {
            ApproxAxis::Fixed { value } => (0..num_coords)
                .map(|_| value)
                .collect::<Vec<_>>()
                .into_iter(),
            ApproxAxis::Uniform { reverse } => {
                let increment = Scalar::from(1. / (num_coords as f64 + 1.));

                let mut coords = (0..num_coords)
                    .map(|i| increment * (i + 1) as f64)
                    .collect::<Vec<_>>();

                if reverse {
                    coords.reverse();
                }

                coords.into_iter()
            }
        }
    }
}
