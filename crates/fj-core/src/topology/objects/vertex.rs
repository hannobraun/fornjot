/// # A vertex that identifies a point in space
///
/// ## Purpose
///
/// Vertices are referenced by [`HalfEdge`]s. They are topological objects,
/// which means that their purpose is to define how parts of a shape relate to
/// each other. They *identify* a point in space, but they do not define its
/// position.
///
/// In fact, this struct does not contain any data at all which could define
/// anything. As such, [`Vertex`] is solely intended to be used through a
/// [`Handle`], which provides the vertex with a unique identity, allowing it
/// to do its job.
///
/// Having a unique identity for a point in space is very valuable, as we can't
/// just rely on the value of points to compare them. If two half-edges connect,
/// we expect them to connect at a single point, not two points that are very
/// close to each other.
///
/// Due to the realities of computing (and specifically, the realities of
/// computing floating-point numbers), we might very well end up *thinking* that
/// our code computed a single point, when in fact it does not, which only shows
/// up outside of our testing environment. This can be a pervasive source of
/// bugs, if left unchecked.
///
/// With [`Vertex`], we can provide a unique identity to each point where it is
/// computed. This allows [validation code](crate::validate) to exist, which can
/// identify where our code generates multiple distinct points that might end up
/// in slightly different positions in a real-life scenario.
///
///
/// ## Positions
///
/// (Warning: If the following information is relevant to you, please double-
/// check it by looking at the code that this section references. Everything
/// here is true at the time of writing, but there are planned changes which
/// could make this section obsolete.)
///
/// A vertex can exist in multiple distinct spaces, and can thus have multiple
/// positions. Those positions are defined by the objects that reference the
/// vertex. How exactly that happens depends on the overall shape.
///
/// If the shape is defined by a [`Sketch`], it is 2D-only. The (2D) position of
/// the vertex will then be defined by the half-edges that reference it.
/// Validation code can make sure that those redundant definitions don't result
/// in wildly different values.
///
/// If the shape is defined by a [`Solid`], then it it 3-dimensional. The
/// referencing half-edges still define the surface-local 2D positions of the
/// vertex. Since such half-edges could meet in the same surface, or exist where
/// multiple surfaces meet, these positions could end up being in one or more
/// surfaces.
///
/// The corresponding [`Surface`] objects can then be used to convert those 2D
/// positions into global 3D positions.
///
/// As you might have noted, in each case, we still have redundant definitions
/// of the vertex position, and might end up with multiple values for the
/// position that are not exactly equal. However, since we know these positions
/// belong to the same vertex, this is not a problem.
///
/// Validation code can make sure that the actual values are very close
/// together, and where we lose this identity information(when generating a
/// triangle mesh for a file export, for example), we can choose exactly one of
/// those values.
///
///
/// ## Equality
///
/// `Vertex` contains no data and exists purely to be referenced via a `Handle`,
/// where `Handle::id` can be used to compare different instances of it.
///
/// If `Vertex` had `Eq`/`PartialEq` implementations, it containing no data
/// would mean that all instances of `Vertex` would be considered equal. This
/// would be very error-prone.
///
/// If you need to reference a `Vertex` from a struct that needs to derive
/// `Eq`/`Ord`/..., you can use `HandleWrapper<Vertex>` to do that. It will
/// use `Handle::id` to provide those `Eq`/`Ord`/... implementations.
///
/// [`HalfEdge`]: crate::topology::HalfEdge
/// [`Handle`]: crate::storage::Handle
/// [`Sketch`]: crate::topology::Sketch
/// [`Solid`]: crate::topology::Solid
/// [`Surface`]: crate::topology::Surface
#[derive(Clone, Debug, Default)]
pub struct Vertex {}

impl Vertex {
    /// Construct a `Vertex`
    pub fn new() -> Self {
        Self::default()
    }
}
