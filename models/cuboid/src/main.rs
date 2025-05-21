use clap::Parser;
use fj::{Args, Instance};

#[derive(Parser)]
struct Parameters {
    /// Size of the cuboid, as a comma-separated vector `x,y,z`
    #[arg(long, value_parser = parse_vector_3, default_value = "1,1,1")]
    size: [f64; 3],

    #[command(flatten)]
    fj: Args,
}

fn parse_vector_3(arg: &str) -> anyhow::Result<[f64; 3]> {
    Ok(arg
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<f64>, _>>()?
        .as_slice()
        .try_into()?)
}

fn main() -> anyhow::Result<()> {
    let mut fj = Instance::new();
    let params = Parameters::parse();

    let model = cuboid::model(params.size, &mut fj.core);
    fj.process_model_args(&model, params.fj)?;

    Ok(())
}
