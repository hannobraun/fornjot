//! API for checking compatibility between the Fornjot app and a model

/// The Fornjot package version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
///
/// This is just the version specified in the Cargo package, which will stay
/// constant between releases, even though changes are made throughout. A match
/// of this version does not conclusively determine that the app and a model are
/// compatible.
pub static VERSION_PKG: &str = env!("FJ_VERSION_PKG");

/// The full Fornjot version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
pub static VERSION_FULL: &str = env!("FJ_VERSION_FULL");
