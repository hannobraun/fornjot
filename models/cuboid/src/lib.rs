use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let x: f64 = args
        .get("x")
        .unwrap_or(&"3.0".to_owned())
        .parse()
        .expect("Could not parse parameter `x`");
    let y: f64 = args
        .get("y")
        .unwrap_or(&"2.0".to_owned())
        .parse()
        .expect("Could not parse parameter `y`");
    let z: f64 = args
        .get("z")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .expect("Could not parse parameter `z`");

    #[rustfmt::skip]
    let rectangle = fj::Sketch::from_points(vec![
        [-x / 2., -y / 2.],
        [ x / 2., -y / 2.],
        [ x / 2.,  y / 2.],
        [-x / 2.,  y / 2.],
    ]).with_color([100,255,0,200]);

    let cuboid = fj::Sweep::from_path(rectangle.into(), [0., 0., z]);

    cuboid.into()
}
