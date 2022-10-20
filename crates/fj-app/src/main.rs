//! # Fornjot Application
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! Together with the [`fj`] library, this application forms the part of Fornjot
//! that is relevant to end users. Please refer to the [Fornjot repository] for
//! usage examples.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [`fj`]: https://crates.io/crates/fj
//! [Fornjot repository]: https://github.com/hannobraun/Fornjot

mod args;
mod config;

use std::path::PathBuf;

use anyhow::{anyhow, Context as _};
use fj_export::export;
use fj_host::{Model, Parameters, Watcher};
use fj_interop::status_report::StatusReport;
use fj_operations::shape_processor::ShapeProcessor;
use fj_window::run::run;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;

use crate::{args::Args, config::Config};

fn main() -> anyhow::Result<()> {
    let mut status = StatusReport::new();
    // Respect `RUST_LOG`. If that's not defined or erroneous, log warnings and
    // above.
    //
    // It would be better to fail, if `RUST_LOG` is erroneous, but I don't know
    // how to distinguish between that and the "not defined" case.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("WARN")),
        )
        .event_format(format().pretty())
        .init();

    let args = Args::parse();
    let config = Config::load()?;
    let path = config.default_path.unwrap_or_else(|| PathBuf::from(""));
    let parameters = args.parameters.unwrap_or_else(Parameters::empty);
    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };

    let path_of_model = path.canonicalize().unwrap_or_default();

    let model = if let Some(model) =
        args.model.or(config.default_model).as_ref()
    {
        let mut model_path = path;
        model_path.push(model);
        Model::from_path(model_path.clone(), parameters).with_context(|| {
            if path_of_model.as_os_str().is_empty() {
                format!(
                    "Model is not defined, can't find model defined inside the default-model also, add model like \n cargo run -- -m {}", model.display()
                )
            } else {
                format!(
                "Failed to load model: {0}\ninside default models directory: '{1}'\nCan mainly caused by: \n1. Model '{2}' can not be found inside '{1}'\n2.'{2}' can be mis-typed see inside '{1}' for a match\n3. Define model is '{2}' couldn\'t be found ((defined in command-line arguments))", model_path.display(), path_of_model.display(), model.display()
            )
        }
        })?
    } else {
        return Err(anyhow!(
            "You must specify a model to start Fornjot.\n\
            - Pass a model as a command-line argument. See `fj-app --help`.\n\
            - Specify a default model in the configuration file."
        ));
    };

    if let Some(export_path) = args.export {
        // export only mode. just load model, process, export and exit

        let shape = model.load_once(&mut status)?;
        let shape = shape_processor.process(&shape)?;

        export(&shape.mesh, &export_path)?;

        return Ok(());
    }

    let invert_zoom = config.invert_zoom.unwrap_or(false);

    let watcher = Watcher::watch_model(model)?;
    run(watcher, shape_processor, status, invert_zoom)?;

    Ok(())
}
