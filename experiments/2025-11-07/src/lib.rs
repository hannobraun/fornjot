pub mod helpers;
pub mod math;
pub mod storage;

use crate::{helpers::Orientation, math::Point, storage::Handle};

/// # A solid body
///
/// Solid bodies are bounded regions within a global and implicit 3D space.
///
/// ## Validation Checks
///
/// - All coincident [`LocalFace`]s within the boundary must be representations
///   of the same [`Face`].
/// - The orientation of all [`LocalFace`]s within the boundary must be
///   consistent.
/// - The boundary forms a closed shell with no holes and no overlap.
/// - All coincident [`LocalEdge`]s must be representations of the same
///   [`Edge`].
/// - All coincident [`LocalCurve`]s must be representations of the same
///   [`Curve`].
/// - All coincident [`LocalVertex`] instances within within the `Edge` must be
///   representations of the same [`Vertex`].
pub struct Solid {
    /// # The boundary of this solid
    ///
    /// The orientation of these `LocalFace`s defines the orientation of the
    /// `Solid`. Their front side points to the _outside_ of the `Solid`, hence
    /// their back side points _inside_.
    pub boundary: Vec<LocalFace>,
}

/// # The representation of a [`Face`] that is local to a [`Solid`]
///
/// All coincident `LocalFace`s within a [`Solid`] must be local representations
/// of the same [`Face`].
///
/// Coincident `LocalFace`s of a [`Solid`] (which must share a [`Face`]) must
/// necessarily have opposite orientation, or otherwise that [`Solid`] would not
/// be valid. This necessitates the distinction between [`Face`] and
/// `LocalFace`.
pub struct LocalFace {
    /// # The face that this `LocalFace` is the local representation of
    pub face: Handle<Face>,

    /// # The orientation of this `LocalFace`
    ///
    /// This is in relation to its [`Face`]'s nominal orientation.
    pub orientation: Orientation,
}

/// # A face
///
/// Faces are bounded regions within a two-dimensional [`Surface`].
///
/// ## Validation
///
/// - All coincident [`LocalEdge`]s within within the face must be
///   representations of the same [`Edge`].
/// - The orientation of all [`LocalEdge`]s within the `Face` must be
///   consistent.
/// - The boundary forms a closed cycle, with not gaps and no overlap.
/// - All coincident [`LocalVertex`] instances within within the `Edge` must be
///   representations of the same [`Vertex`].
pub struct Face {
    /// # The surface that this `Face` is defined on
    pub surface: Handle<Surface>,

    /// # The boundary of this `Face`
    ///
    /// The orientation of the boundary defines the nominal orientation of the
    /// `Face`. The side on which the `LocalEdge`s form a counter-clockwise
    /// cycle is the nominal front side.
    ///
    /// The orientation of a face can only be nominal, as orientation is only
    /// meaningful in [`LocalFace`]s. Those define their own orientation in
    /// relation to this nominal orientation.
    pub boundary: Vec<LocalEdge>,
}

/// # The representation of an [`Edge`] that is local to a [`Face`]
///
/// All coincident `LocalEdge`s within a [`Face`] or [`Solid`] must be local
/// representations of the same [`Edge`].
///
/// Coincident `LocalEdge`s of a [`Face`] or [`Solid`] (which must share an
/// [`Edge`]) must necessarily have opposite orientation, or otherwise that
/// [`Face`]/[`Solid`] would not be valid. This necessitates the distinction
/// between [`Edge`] and `LocalEdge`.
pub struct LocalEdge {
    /// # The edge that this `LocalEdge` is the local representation of
    pub edge: Handle<Edge>,

    /// # The curve that this `LocalEdge` is defined on
    ///
    /// Since a `LocalCurve` is local to a [`Surface`], a single `LocalCurve`
    /// can still be shared between multiple `LocalEdge`s.
    pub curve: Handle<LocalCurve>,

    /// # The orientation of this `LocalEdge`
    ///
    /// This is in relation to its [`Edge`]'s nominal orientation.
    pub orientation: Orientation,
}

/// # An edge
///
/// Edges are bounded regions within a one-dimensional [`Curve`]. That [`Curve`]
/// is not referenced here, as it is available through an `Edge`'s local
/// representations ([`LocalEdge`]), via [`LocalCurve`].
///
/// ## Validation
///
/// - All coincident [`LocalVertex`] instances within within the `Edge` must be
///   representations of the same [`Vertex`].
pub struct Edge {
    /// # The boundary of this `Edge`
    ///
    /// The order of the vertices defines the nominal orientation of the `Edge`.
    /// This orientation can only be nominal, as orientation is only meaningful
    /// in [`LocalEdge`]s. Those define their own orientation in relation to
    /// this nominal orientation.
    pub boundary: [LocalVertex; 2],
}

/// # The representation of a [`Vertex`] that is local to an [`Edge`]
///
/// All coincident `LocalVertex` instances within an [`Edge`], [`Face`], or
/// [`Solid`] must be local representations of the same [`Vertex`].
pub struct LocalVertex {
    /// # The vertex that this `LocalVertex` is the local representation of
    pub vertex: Handle<Vertex>,

    /// # The position of the `LocalVertex` on a curve
    pub position: Point<1>,
}

/// # A vertex
///
/// This struct has no fields. It only exists to define the identity of a
/// vertex, which is achieved through the use of [`Handle`].
///
/// The position of a `Vertex` is (redundantly) defined by the [`LocalVertex`]
/// instances that are its representations.
pub struct Vertex {}

/// # The representation of a curve that is local to a [`Surface`]
///
/// All coincident `LocalCurves`s within a [`Face`] or [`Solid`] must be local
/// representations of the same [`Curve`].
pub struct LocalCurve {
    /// # The curve that this `LocalCurve` is the local representation of
    pub curve: Handle<Curve>,

    /// # A placeholder for the curve geometry
    pub geometry: (),
}

/// # A curve
///
/// This struct has no fields. It only exists to define the identity of a curve,
/// which is achieved through the use of [`Handle`].
///
/// The geometry of a `Curve` is (redundantly) defined by the [`LocalCurve`]
/// instances that are its representations.
pub struct Curve {}

/// # A surface
pub struct Surface {
    /// # A placeholder for the surface geometry
    pub geometry: (),
}
