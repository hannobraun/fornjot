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
mod path;

use std::{env, error::Error};

use anyhow::{anyhow, Context};
use fj_export::export;
use fj_host::Parameters;
use fj_operations::shape_processor::ShapeProcessor;
use fj_window::run::run;
use path::ModelPath;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;

use crate::{args::Args, config::Config};

fn main() -> anyhow::Result<()> {
    // Respect `RUST_LOG`. If that's not defined, log warnings and above. Fail if it's erroneous.
    tracing_subscriber::fmt()
        .with_env_filter(try_default_env_filter()?)
        .event_format(format().pretty())
        .init();

    let args = Args::parse();
    let config = Config::load()?;
    let model_path = ModelPath::from_args_and_config(&args, &config);
    let parameters = args.parameters.unwrap_or_else(Parameters::empty);
    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };

    let model = model_path.map(|m| m.load_model(parameters)).transpose()?;

    if let Some(export_path) = args.export {
        // export only mode. just load model, process, export and exit

        let evaluation = model.with_context(no_model_error)?.evaluate()?;
        let shape = shape_processor.process(&evaluation.shape)?;

        export(&shape.mesh, &export_path)?;

        return Ok(());
    }

    let invert_zoom = config.invert_zoom.unwrap_or(false);
    run(model, shape_processor, invert_zoom)?;

    Ok(())
}

fn no_model_error() -> anyhow::Error {
    anyhow!(
        "You must specify a model to start Fornjot in export only mode.\n\
        - Pass a model as a command-line argument. See `fj-app --help`.\n\
        - Specify a default model in the configuration file."
    )
}

fn try_default_env_filter() -> anyhow::Result<EnvFilter> {
    let env_filter = EnvFilter::try_from_default_env();

    match env_filter {
        Ok(env_filter) => Ok(env_filter),

        Err(err) => {
            if let Some(kind) = err.source() {
                if let Some(env::VarError::NotPresent) =
                    kind.downcast_ref::<env::VarError>()
                {
                    return Ok(EnvFilter::new("WARN"));
                }
            } else {
                // `tracing_subscriber::filter::FromEnvError` currently returns a source
                // in all cases.
                unreachable!()
            }

            Err(anyhow!(
                "There was an error parsing the RUST_LOG environment variable."
            ))
        }
    }
}
