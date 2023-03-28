use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
    ptr::NonNull,
    str,
};

use fj::{
    abi::{self, SelfSerializing},
    version::Version,
};
use fj_operations::shape_processor;
use tracing::debug;

use crate::{platform::HostPlatform, Parameters};

/// Represents a Fornjot model
pub struct Model {
    src_path: PathBuf,
    lib_path: PathBuf,
    manifest_path: PathBuf,
    parameters: Parameters,
}

impl Model {
    /// Initialize the model using the path to its crate
    ///
    /// The path expected here is the root directory of the model's Cargo
    /// package, that is the folder containing `Cargo.toml`.
    pub fn new(
        path: impl AsRef<Path>,
        parameters: Parameters,
    ) -> Result<Self, Error> {
        let path = path.as_ref();

        let crate_dir = path.canonicalize()?;

        let metadata = cargo_metadata::MetadataCommand::new()
            .current_dir(&crate_dir)
            .exec()?;

        let pkg = package_associated_with_directory(&metadata, &crate_dir)?;
        let src_path = crate_dir.join("src");

        let lib_path = {
            let name = pkg.name.replace('-', "_");
            let file = HostPlatform::lib_file_name(&name);
            let target_dir =
                metadata.target_directory.clone().into_std_path_buf();
            target_dir.join("debug").join(file)
        };

        Ok(Self {
            src_path,
            lib_path,
            manifest_path: pkg.manifest_path.as_std_path().to_path_buf(),
            parameters,
        })
    }

    /// Access the path that needs to be watched for changes
    pub fn watch_path(&self) -> PathBuf {
        self.src_path.clone()
    }

    /// Evaluate the model
    pub fn evaluate(&self) -> Result<Evaluation, Error> {
        let manifest_path = self.manifest_path.display().to_string();

        let cargo_output = Command::new("cargo")
            .arg("rustc")
            .args(["--manifest-path", &manifest_path])
            .args(["--crate-type", "cdylib"])
            .output()?;

        if !cargo_output.status.success() {
            let output =
                String::from_utf8(cargo_output.stderr).unwrap_or_else(|_| {
                    String::from("Failed to fetch command output")
                });

            return Err(Error::Compile { output });
        }

        let seconds_taken = str::from_utf8(&cargo_output.stderr)
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .trim();

        let mut warnings = None;

        // So, strictly speaking this is all unsound:
        // - `Library::new` requires us to abide by the arbitrary requirements
        //   of any library initialization or termination routines.
        // - `Library::get` requires us to specify the correct type for the
        //   model function.
        // - The model function itself is `unsafe`, because it is a function
        //   from across an FFI interface.
        //
        // Typical models won't have initialization or termination routines (I
        // think), should abide by the `ModelFn` signature, and might not do
        // anything unsafe. But we have no way to know that the library the user
        // told us to load actually does (I think).
        //
        // I don't know of a way to fix this. We should take this as motivation
        // to switch to a better technique:
        // https://github.com/hannobraun/Fornjot/issues/71
        let shape = unsafe {
            let lib = libloading::Library::new(&self.lib_path)
                .map_err(Error::LoadingLibrary)?;

            let version_pkg_host = fj::version::VERSION_PKG.to_string();

            let version_pkg_model: libloading::Symbol<*const Version> =
                lib.get(b"VERSION_PKG").map_err(Error::LoadingVersion)?;
            let version_pkg_model = (**version_pkg_model).to_string();

            debug!(
                "Comparing package versions (host: {}, model: {})",
                version_pkg_host, version_pkg_model
            );
            if version_pkg_host != version_pkg_model {
                let host = String::from_utf8_lossy(version_pkg_host.as_bytes())
                    .into_owned();
                let model = version_pkg_model;

                return Err(Error::VersionMismatch { host, model });
            }

            let version_full_host = fj::version::VERSION_FULL.to_string();

            let version_full_model: libloading::Symbol<*const Version> =
                lib.get(b"VERSION_FULL").map_err(Error::LoadingVersion)?;
            let version_full_model = (**version_full_model).to_string();

            debug!(
                "Comparing full versions (host: {}, model: {})",
                version_full_host, version_full_model
            );
            if version_full_host != version_full_model {
                let host =
                    String::from_utf8_lossy(version_full_host.as_bytes())
                        .into_owned();
                let model = version_full_model;

                warnings =
                    Some(format!("{}", Error::VersionMismatch { host, model }));
            }

            let construct: libloading::Symbol<abi::ConstructModelFunction> =
                lib.get(abi::CONSTRUCT_FUNCTION_NAME.as_bytes())
                    .map_err(Error::LoadingInit)?;

            let free: libloading::Symbol<abi::FreeModelFunction> = lib
                .get(abi::FREE_FUNCTION_NAME.as_bytes())
                .map_err(Error::LoadingInit)?;

            let parameters = self
                .parameters
                .0
                .serialize()
                .map_err(Error::ParameterSerialization)?;

            let parameter_length = parameters.len();
            let parameters_ptr = parameters.as_ptr();

            let mut shape_data: *mut u8 = std::ptr::null_mut();
            let shape_length = construct(
                &mut shape_data as *mut _,
                parameters_ptr,
                parameter_length,
            );

            let model_result = {
                let shape_data =
                    NonNull::new(shape_data).ok_or(Error::NoModel)?;
                let shape_payload = std::slice::from_raw_parts(
                    shape_data.as_ptr(),
                    shape_length,
                );

                fj::abi::ModelResult::deserialize(shape_payload)
            };

            // Free the payload before we check for an error.
            free(shape_data);

            let model_result =
                model_result.map_err(Error::ModelDeserialization)?;
            match model_result {
                abi::ModelResult::Panic(panic) => {
                    return Err(Error::Shape(panic))
                }
                abi::ModelResult::Error(error) => {
                    return Err(Error::Shape(error))
                }
                abi::ModelResult::Ok(model) => model.shape,
            }
        };

        Ok(Evaluation {
            shape,
            compile_time: seconds_taken.into(),
            warning: warnings,
        })
    }
}

/// The result of evaluating a model
///
/// See [`Model::evaluate`].
#[derive(Debug)]
pub struct Evaluation {
    /// The shape
    pub shape: fj::Shape,

    /// The time it took to compile the shape, from the Cargo output
    pub compile_time: String,

    /// Warnings
    pub warning: Option<String>,
}

fn package_associated_with_directory<'m>(
    metadata: &'m cargo_metadata::Metadata,
    dir: &Path,
) -> Result<&'m cargo_metadata::Package, Error> {
    for pkg in metadata.workspace_packages() {
        let crate_dir = pkg
            .manifest_path
            .parent()
            .and_then(|p| p.canonicalize().ok());

        if crate_dir.as_deref() == Some(dir) {
            return Ok(pkg);
        }
    }

    Err(ambiguous_path_error(metadata, dir))
}

fn ambiguous_path_error(
    metadata: &cargo_metadata::Metadata,
    dir: &Path,
) -> Error {
    let mut possible_paths = Vec::new();

    for id in &metadata.workspace_members {
        let cargo_toml = &metadata[id].manifest_path;
        let crate_dir = cargo_toml
            .parent()
            .expect("A Cargo.toml always has a parent");
        // Try to make the path relative to the workspace root so error messages
        // aren't super long.
        let simplified_path = crate_dir
            .strip_prefix(&metadata.workspace_root)
            .unwrap_or(crate_dir);

        possible_paths.push(simplified_path.into());
    }

    Error::AmbiguousPath {
        dir: dir.to_path_buf(),
        possible_paths,
    }
}

/// An error that can occur when loading or reloading a model
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error loading model library
    #[error(
        "Failed to load model library\n\
        This might be a bug in Fornjot, or, at the very least, this error \
        message should be improved. Please report this!"
    )]
    LoadingLibrary(#[source] libloading::Error),

    /// Error loading Fornjot version that the model uses
    #[error(
        "Failed to load the Fornjot version that the model uses\n\
        - Is your model using the `fj` library? All models must!\n\
        - Was your model created with a really old version of Fornjot?"
    )]
    LoadingVersion(#[source] libloading::Error),

    /// Error loading the model's `init` function
    #[error(
        "Failed to load the model's `init` function\n\
        - Did you define a model function using `#[fj::model]`?"
    )]
    LoadingInit(#[source] libloading::Error),

    /// Host version and model version do not match
    #[error("Host version ({host}) and model version ({model}) do not match")]
    VersionMismatch {
        /// The host version
        host: String,

        /// The model version
        model: String,
    },

    /// Model failed to compile
    #[error("Error compiling model\n{output}")]
    Compile {
        /// The compiler output
        output: String,
    },

    /// I/O error while loading the model
    #[error("I/O error while loading model")]
    Io(#[from] io::Error),

    /// An error was returned from [`fj::models::Model::shape()`].
    #[error("Unable to determine the model's geometry")]
    Shape(String),

    /// An error was returned from
    /// [`fj_operations::shape_processor::ShapeProcessor::process()`].
    #[error("Shape processing error")]
    ShapeProcessor(#[from] shape_processor::Error),

    /// Error while watching the model code for changes
    #[error("Error watching model for changes")]
    Notify(#[from] notify::Error),

    /// An error occurred while trying to use evaluate
    /// [`cargo_metadata::MetadataCommand`].
    #[error("Unable to determine the crate's metadata")]
    CargoMetadata(#[from] cargo_metadata::Error),

    /// The user pointed us to a directory, but it doesn't look like that was
    /// a crate root (i.e. the folder containing `Cargo.toml`).
    #[error(
        "It doesn't look like \"{}\" is a crate directory. Did you mean one of {}?",
        dir.display(),
        possible_paths.iter().map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )]
    AmbiguousPath {
        /// The model directory supplied by the user.
        dir: PathBuf,
        /// The directories for each crate in the workspace, relative to the
        /// workspace root.
        possible_paths: Vec<PathBuf>,
    },

    /// This should never happen but we need a fail safe.
    /// Something about our parameter hash map couldn't be serialized.
    #[error("Failed to serialize parameters for model")]
    ParameterSerialization(postcard::Error),

    /// This error actually is very possible.
    /// If the client serialized its model incorrectly.
    #[error("Failed to deserialize model from client")]
    ModelDeserialization(postcard::Error),

    /// The model function gave us a null pointer. There's
    /// literally nothing for us to work with here.
    #[error("Client returned nothing")]
    NoModel,
}
