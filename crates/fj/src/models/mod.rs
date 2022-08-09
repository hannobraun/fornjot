//! Interfaces used when defining models.

mod context;
mod host;
mod macros;
mod metadata;
mod model;
mod types;

pub use self::{
    context::{
        Context, ContextError, ContextExt, MissingArgument, ParseFailed,
    },
    host::{Host, HostExt},
    metadata::{ArgumentMetadata, Metadata, ModelMetadata},
    model::Model,
};

/// The error type used in this module.
pub type Error = abi_stable::std_types::RBoxError;

/// Internal implementation details for the host-guest interface.
///
/// Note that the vast majority of this module is just providing FFI-safe
/// versions of common `std` types (e.g. `Vec`, `String`, and `Box<dyn Error>`),
/// or FFI-safe trait objects.
///
/// If the macro generated the wrong code, this doctest would fail.
///
/// ```rust
/// use fj::models::{
///     internal::{self, INIT_FUNCTION_NAME, RResult, RBoxError},
///     Metadata,
/// };
///
/// fj::register_model!(|_| {
///     Ok(Metadata::new("My Model", "1.2.3"))
/// });
///
/// mod x {
///     use fj::models::{Metadata, internal::{Host, RResult, RBoxError}};
///
///     extern "C" {
///         pub fn fj_model_init(_: Host<'_>) -> RResult<Metadata, RBoxError>;
///     }
/// }
///
/// // make sure our function has the right signature
/// let func: internal::InitFunction = fj_model_init;
///
/// // We can also make sure the unmangled name is correct by calling it.
///
/// let metadata: Metadata = unsafe {
///     let mut host = Host;
///     let mut host = internal::Host::new(&mut host);
///     x::fj_model_init(host).unwrap().into()
/// };
///
/// assert_eq!(metadata.name, "My Model");
///
/// struct Host;
/// impl fj::models::Host for Host {
///     fn register_boxed_model(&mut self, model: internal::Model) { todo!() }
/// }
/// ```
pub mod internal {
    pub use crate::models::{
        context::Context_trait, host::Host_trait, model::Model_trait, types::*,
    };
    pub use abi_stable::std_types::{
        RBoxError, RErr, RNone, ROk, ROption, RResult, RSome,
    };
}
