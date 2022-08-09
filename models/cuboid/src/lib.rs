#[fj::model]
pub fn model(
    #[param(default = 3.0)] x: f64,
    #[param(default = 2.0)] y: f64,
    #[param(default = 1.0)] z: f64,
) -> fj::Shape {
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
