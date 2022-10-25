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

use anyhow::Context as _;
use fj_export::export;
use fj_host::Parameters;
use fj_operations::shape_processor::ShapeProcessor;
use fj_window::run::run;
use path::ModelPath;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;

use crate::{args::Args, config::Config};

fn main() -> anyhow::Result<()> {
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
    let path = ModelPath::from_args_and_config(&args, &config)?;
    let parameters = args.parameters.unwrap_or_else(Parameters::empty);
    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };

    let model = {
        let model_path = path.path();
        path.load_model(parameters).with_context(|| {
            format!(
                "Failed to load model: {0}\ninside default models directory: '{1}'\nCan mainly caused by: \n1. Model '{2}' can not be found inside '{1}'\n2.'{2}' can be mis-typed see inside '{1}' for a match\n3. Define model is '{2}' couldn\'t be found ((defined in command-line arguments))", model_path.display(), path.default_path().display(), path.model_path_without_default().display()
            )
        })?
    };

    if let Some(export_path) = args.export {
        // export only mode. just load model, process, export and exit

        let shape = model.load()?;
        let shape = shape_processor.process(&shape.shape)?;

        export(&shape.mesh, &export_path)?;

        return Ok(());
    }

    let invert_zoom = config.invert_zoom.unwrap_or(false);

    run(model, shape_processor, invert_zoom)?;

    Ok(())
}
