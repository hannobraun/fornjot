// TASK: Consider simplifying this module:
//       - Do away with indices. Just convert `Mesh` into a `Vec<Triangle>`,
//         basically.
//       - Merge `Mesh` and `MeshMaker`.
//
//       I don't think the complexity here is pulling its weight. All it does is
//       save a bit of RAM. And `graphics::Vertices` is basically duplicating
//       the the logic here anyway, because it uses a different vertex type.

/// An index that refers to a vertex
///
/// See [`Mesh`].
///
/// Since this type is used to index into a [`Vec`], [`usize`] would seem to be the
/// natural type to use here. We're using this for computer graphics, however,
/// and for that we need a type with defined size.
pub type Index = u32;
