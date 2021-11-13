/// An index that refers to a vertex
///
/// See [`Mesh`].
///
/// Since this type is used to index into a [`Vec`], [`usize`] would seem to be the
/// natural type to use here. We're using this for computer graphics, however,
/// and for that we need a type with defined size.
pub type Index = u32;
