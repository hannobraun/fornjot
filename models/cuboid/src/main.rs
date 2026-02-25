use clap::Parser;

#[derive(Parser)]
struct CuboidArgs {
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
    fj: fj::Arguments,
}

fn main() -> fj::Result {
    let args = CuboidArgs::parse();

    let model = cuboid::model([args.x, args.y, args.z]);
    fj::process_model(model, args.fj)?;

    Ok(())
}
