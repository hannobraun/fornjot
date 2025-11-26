use fj::math::Point;

fn main() -> anyhow::Result<()> {
    let point = Point::from([0.]);
    println!("{point:?}");
    Ok(())
}
