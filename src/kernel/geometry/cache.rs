/// The geometry cache
///
/// Due to floating point accuracy issues, it is error-prone to refer to
/// geometry directly.
///
/// Since any object can be referenced by multiple other objects (for example, a
/// vertex can be shared by multiple edges), storing such a reference as
/// geometry (for example, storing a vertex as a point in space) risks computing
/// those same objects in different ways, leading to different results.
///
/// This can result in the same objects being mistaken for different ones, due
/// to slight differences.
///
/// This cache presents a principled approach to preventing this: Each geometric
/// object is computed only once, and is only ever referred to by the handle
/// returned from this cache.
///
/// The alternative would be to be really careful, everywhere, and plug holes as
/// they are found.
#[allow(unused)]
pub struct Cache;
