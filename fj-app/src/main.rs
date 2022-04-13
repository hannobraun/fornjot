mod args;
mod camera;
mod config;
mod graphics;
mod input;
mod run;
mod window;

use std::path::PathBuf;

use anyhow::anyhow;
use fj_export::export;
use fj_host::{Model, Parameters};
use fj_operations::shape_processor::ShapeProcessor;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;

use crate::{args::Args, config::Config, run::run};

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

    let mut path = config.default_path.unwrap_or_else(|| PathBuf::from(""));
    let model = args.model.or(config.default_model).ok_or_else(|| {
        anyhow!(
            "No model specified, and no default model configured.\n\
                Specify a model by passing `--model path/to/model`."
        )
    })?;
    path.push(model);

    let model = Model::from_path(path, config.target_dir)?;
    let parameters = args.parameters.unwrap_or_else(Parameters::empty);

    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };

    if let Some(path) = args.export {
        let shape = model.load_once(&parameters)?;
        let shape = shape_processor.process(&shape);

        export(&shape.mesh, &path)?;

        return Ok(());
    }

    let watcher = model.load_and_watch(parameters)?;
    run(watcher, shape_processor)?;

    Ok(())
}
