use anyhow::Result;
use clap::Parser;
use fj::{Args, Instance};

#[derive(Parser)]
struct Parameters {
    /// Outer radius of the spacer
    #[arg(long, default_value = "1.0")]
    outer_radius: f64,

    /// Inner radius of the spacer
    #[arg(long, default_value = "0.5")]
    inner_radius: f64,

    /// Height of the spacer
    #[arg(long, default_value = "1.0")]
    height: f64,

    #[command(flatten)]
    fj: Args,
}

fn main() -> Result<()> {
    let mut fj = Instance::new();
    let params = Parameters::parse();

    let model = spacer::model(
        params.outer_radius,
        params.inner_radius,
        params.height,
        &mut fj.core,
    );
    fj.process_model_args(&model, params.fj)?;

    Ok(())
}
