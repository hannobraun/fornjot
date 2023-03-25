//! Internal implementation details for the host-guest interface.
//!
//! Note that the vast majority of this module is just providing FFI-safe
//! versions of common `std` types (e.g. `Vec`, `String`, and `Box<dyn Error>`),
//! or FFI-safe trait objects.
//!
/// If the macro generated the wrong code, this doctest would fail.
///
/// ```rust
/// use fj::{abi::INIT_FUNCTION_NAME, models::Metadata};
///
/// fj::register_model!(|_| {
///     Ok(Metadata::new("My Model", "1.2.3"))
/// });
///
/// mod x {
///     extern "C" {
///         pub fn fj_model_init(_: *mut fj::abi::Host<'_>) -> fj::abi::InitResult;
///     }
/// }
///
/// // make sure our function has the right signature
/// let func: fj::abi::InitFunction = fj_model_init;
///
/// // We can also make sure the unmangled name is correct by calling it.
///
/// let metadata: fj::models::Metadata = unsafe {
///     let mut host = Host;
///     let mut host = fj::abi::Host::from(&mut host);
///     x::fj_model_init(&mut host).unwrap().into()
/// };
///
/// assert_eq!(metadata.name, "My Model");
///
/// struct Host;
/// impl fj::models::Host for Host {
///     fn register_boxed_model(&mut self, model: Box<dyn fj::models::Model>) { todo!() }
/// }
/// ```
pub mod ffi_safe;
use std::{any::Any, fmt::Display, panic, sync::Mutex};

pub use crate::models::Model;

pub trait SelfSerializing<'a>:
    serde::Serialize + serde::Deserialize<'a> + std::marker::Sized
{
    fn serialize(&self) -> postcard::Result<Vec<u8>> {
        postcard::to_allocvec(self)
    }

    fn deserialize(data: &'a [u8]) -> postcard::Result<Self> {
        postcard::from_bytes(data)
    }
}

/// The result of attempting to build a model.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ModelResult {
    /// The client code panicked.
    Panic(String),

    /// A more general error. It didn't panic but the model couldn't be created.
    Error(String),

    /// A model was successfully built.
    Ok(Model),
}

impl<'a> SelfSerializing<'a> for ModelResult {}

impl<'a> SelfSerializing<'a> for std::collections::HashMap<String, String> {}
pub type ArgumentTable<'a> = std::collections::HashMap<String, String>;

/// The signature of the function to construct our model.
/// The model result is expected to be left in an array of memory.
/// We will take ownership of it but not make any other assumptions about how its memory was allocated and will later pass it back to be freed.
pub type ConstructModelFunction =
    unsafe extern "C" fn(*mut *mut u8, *const u8, usize) -> usize;

/// The signature of the function to free our model.
pub type DestroyModelFunction = unsafe extern "C" fn(*mut u8);

/// The name of the function to construct our model.
///
pub const CONSTRUCT_FUNCTION_NAME: &str = "fj_model_construct";

/// The name of the function to free our model.
pub const FREE_FUNCTION_NAME: &str = "fj_model_free";

// Contains details about a panic that we need to pass back to the application from the panic hook.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PanicInfo {
    message: Option<String>,
    location: Option<Location>,
}

impl Display for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .message
            .as_ref()
            .map_or("No error given", |message| message.as_str());

        write!(f, "\"{}\", ", message)?;

        if let Some(location) = self.location.as_ref() {
            write!(f, "{}", location)?;
        } else {
            write!(f, "no location given")?;
        }

        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Location {
    file: String,
    line: u32,
    column: u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

static LAST_PANIC: Mutex<Option<PanicInfo>> = Mutex::new(None);

/// Capturing panics is something Rust really doesn't want you to do, and as such, they make it convoluted.
/// This sets up all the machinery in the background to pull it off.
///
/// It's okay to call this multiple times.
pub fn initialize_panic_handling() {
    panic::set_hook(Box::new(|panic_info| {
        let mut last_panic =
            LAST_PANIC.lock().expect("Panic queue was poisoned."); // FIXME that can probably overflow the stack.
        let message = if let Some(s) =
            panic_info.payload().downcast_ref::<std::string::String>()
        {
            Some(s.as_str())
        } else {
            panic_info.payload().downcast_ref::<&str>().copied()
        }
        .map(|s| s.to_string());

        let location = panic_info.location().map(|location| Location {
            file: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        });

        *last_panic = Some(PanicInfo { message, location });
    }));
}

pub fn on_panic(_payload: Box<dyn Any + Send>) -> String {
    // The payload is technically no longer needed, but I left it there just in case a change to `catch_unwind` made
    // it useful again.
    if let Ok(mut panic_info) = LAST_PANIC.lock() {
        if let Some(panic_info) = panic_info.take() {
            format!("Panic in model: {}", panic_info)
        } else {
            "Panic in model: No details were given.".to_string()
        }
    } else {
        "Panic in model, but due to a poisoned panic queue the information could not be collected."
        .to_string()
    }
}
