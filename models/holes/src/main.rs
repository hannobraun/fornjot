use anyhow::Result;
use clap::Parser;
use fj::{Args, Instance};

#[derive(Parser)]
struct Parameters {
    /// Radius of the holes
    #[arg(long, default_value = "0.25")]
    radius: f64,

    #[command(flatten)]
    fj: Args,
}

fn main() -> Result<()> {
    let mut fj = Instance::new();
    let params = Parameters::parse();

    let model = holes::model(params.radius, &mut fj.core);
    fj.process_model_args(&model, params.fj)?;

    Ok(())
}
