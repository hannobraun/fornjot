use std::f64::consts::PI;

#[fj::model]
pub fn model(
    #[value(default = 5, min = 3)] num_points: u64,
    #[value(default = 1.0, min = 1.0)] r1: f64,
    #[value(default = 2.0, min = 2.0)] r2: f64,
    #[value(default = 1.0)] h: f64,
) -> fj::Shape {
    let num_vertices = num_points * 2;
    let vertex_iter = (0..num_vertices).map(|i| {
        let angle =
            fj::Angle::from_rad(2. * PI / num_vertices as f64 * i as f64);
        let radius = if i % 2 == 0 { r1 } else { r2 };
        (angle, radius)
    });

    // Now that we got that iterator prepared, generating the vertices is just a
    // bit of trigonometry.
    let mut outer = Vec::new();
    let mut inner = Vec::new();
    for (angle, radius) in vertex_iter {
        let (sin, cos) = angle.rad().sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        outer.push([x, y]);
        inner.push([x / 2., y / 2.]);
    }

    let outer = fj::Sketch::from_points(outer);
    let inner = fj::Sketch::from_points(inner);

    let footprint = fj::Difference2d::from_shapes([outer.into(), inner.into()]);

    let star = fj::Sweep::from_path(footprint.into(), [0., 0., h]);

    star.into()
}
