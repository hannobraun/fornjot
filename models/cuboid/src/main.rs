use clap::Parser;

#[derive(Parser)]
struct Parameters {
    /// Size of the cuboid along the x-axis
    #[arg(long, default_value = "3.0")]
    x: f64,

    /// Size of the cuboid along the y-axis
    #[arg(long, default_value = "2.0")]
    y: f64,

    /// Size of the cuboid along the z-axis
    #[arg(long, default_value = "1.0")]
    z: f64,

    #[command(flatten)]
    fj: fj::Args,
}

fn main() -> fj::Result {
    let params = Parameters::parse();

    let tri_mesh = cuboid::model([params.x, params.y, params.z]);
    fj::process_model(tri_mesh, params.fj)?;

    Ok(())
}
