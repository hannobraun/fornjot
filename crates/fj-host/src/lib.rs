//! # Fornjot Model Host
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! The purpose of this library is to load Fornjot models and watch them for
//! changes. Fornjot models are basically plugins that can be loaded into a CAD
//! application. This library is the host for these model plugins.
//!
//! [Fornjot]: https://www.fornjot.app/

#![warn(missing_docs)]

mod parameters;
mod platform;
mod watcher;

pub use self::{parameters::Parameters, watcher::Watcher};

use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
    str,
};

use fj::{abi, version::RawVersion};
use fj_interop::status_report::StatusReport;
use thiserror::Error;

use self::platform::HostPlatform;

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
    pub fn new(path: PathBuf, parameters: Parameters) -> Result<Self, Error> {
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

    pub(crate) fn src_path(&self) -> PathBuf {
        self.src_path.clone()
    }

    /// Load the model once
    ///
    /// The passed arguments are provided to the model. Returns the shape that
    /// the model returns.
    pub fn load_once(
        &self,
        status: &mut StatusReport,
    ) -> Result<fj::Shape, Error> {
        let manifest_path = self.manifest_path.display().to_string();

        let mut command_root = Command::new("cargo");

        let command = command_root
            .arg("rustc")
            .args(["--manifest-path", &manifest_path])
            .args(["--crate-type", "cdylib"]);

        let cargo_output = command.output()?;
        let exit_status = cargo_output.status;

        if exit_status.success() {
            let seconds_taken = str::from_utf8(&cargo_output.stderr)
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1
                .trim();
            status.update_status(
                format!("Model compiled successfully in {seconds_taken}!")
                    .as_str(),
            );
        } else {
            let output = match command.output() {
                Ok(output) => {
                    String::from_utf8(output.stderr).unwrap_or_else(|_| {
                        String::from("Failed to fetch command output")
                    })
                }
                Err(_) => String::from("Failed to fetch command output"),
            };
            status.clear_status();
            status.update_status(&format!(
                "Failed to compile model:\n{}",
                output
            ));
            return Err(Error::Compile);
        }

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
            let lib = libloading::Library::new(&self.lib_path)?;

            let version_pkg: libloading::Symbol<fn() -> RawVersion> =
                lib.get(b"version_pkg")?;

            let version_pkg = version_pkg();
            if fj::version::VERSION_PKG != version_pkg.as_str() {
                let host = String::from_utf8_lossy(
                    fj::version::VERSION_PKG.as_bytes(),
                )
                .into_owned();
                let model =
                    String::from_utf8_lossy(version_pkg.as_str().as_bytes())
                        .into_owned();

                return Err(Error::VersionMismatch { host, model });
            }

            let init: libloading::Symbol<abi::InitFunction> =
                lib.get(abi::INIT_FUNCTION_NAME.as_bytes())?;

            let mut host = Host::new(&self.parameters);

            match init(&mut abi::Host::from(&mut host)) {
                abi::ffi_safe::Result::Ok(_metadata) => {}
                abi::ffi_safe::Result::Err(e) => {
                    return Err(Error::InitializeModel(e.into()));
                }
            }

            let model = host.take_model().ok_or(Error::NoModelRegistered)?;

            model.shape(&host).map_err(Error::Shape)?
        };

        Ok(shape)
    }
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
#[derive(Debug, Error)]
pub enum Error {
    /// Model failed to compile
    #[error("Error compiling model")]
    Compile,

    /// I/O error while loading the model
    #[error("I/O error while loading model")]
    Io(#[from] io::Error),

    /// Failed to load the model's dynamic library
    #[error("Error loading model from dynamic library")]
    LibLoading(#[from] libloading::Error),

    /// Host version and model version do not match
    #[error("Host version ({host}) and model version ({model}) do not match")]
    VersionMismatch {
        /// The host version
        host: String,

        /// The model version
        model: String,
    },

    /// Initializing a model failed.
    #[error("Unable to initialize the model")]
    InitializeModel(#[source] fj::models::Error),

    /// The user forgot to register a model when calling
    /// [`fj::register_model!()`].
    #[error("No model was registered")]
    NoModelRegistered,

    /// An error was returned from [`fj::models::Model::shape()`].
    #[error("Unable to determine the model's geometry")]
    Shape(#[source] fj::models::Error),

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
}

struct Host<'a> {
    args: &'a Parameters,
    model: Option<Box<dyn fj::models::Model>>,
}

impl<'a> Host<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        Self {
            args: parameters,
            model: None,
        }
    }

    pub fn take_model(&mut self) -> Option<Box<dyn fj::models::Model>> {
        self.model.take()
    }
}

impl<'a> fj::models::Host for Host<'a> {
    fn register_boxed_model(&mut self, model: Box<dyn fj::models::Model>) {
        self.model = Some(model);
    }
}

impl<'a> fj::models::Context for Host<'a> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.args.get(name).map(|s| s.as_str())
    }
}
