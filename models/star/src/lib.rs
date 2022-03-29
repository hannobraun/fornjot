use std::{collections::HashMap, f64::consts::PI};

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    // Number of points of the star
    //
    // "Points" in the sense of "pointy ends", not in the sense of geometrical
    // points, or vertices.
    let num_points: u64 = args
        .get("num_points")
        .map(|arg| arg.parse().unwrap())
        .unwrap_or(5);

    // Radius of the circle that all the vertices between the pointy ends are on
    let r1: f64 = args
        .get("r1")
        .map(|arg| arg.parse().unwrap())
        .unwrap_or(1.0);

    // Radius of the circle that all the pointy ends are on
    let r2: f64 = args
        .get("r2")
        .map(|arg| arg.parse().unwrap())
        .unwrap_or(2.0);

    // The height of the star
    let h: f64 = args.get("h").map(|arg| arg.parse().unwrap()).unwrap_or(1.0);

    // We need to figure out where to generate vertices, depending on the number
    // of points the star is supposed to have. Let's generate an iterator that
    // gives us the angle and radius for each vertex.
    let num_vertices = num_points * 2;
    let vertex_iter = (0..num_vertices).map(|i| {
        let angle = 2. * PI / num_vertices as f64 * i as f64;
        let radius = if i % 2 == 0 { r1 } else { r2 };
        (angle, radius)
    });

    // Now that we got that iterator prepared, generating the vertices is just a
    // bit of trigonometry.
    let mut outer = Vec::new();
    let mut inner = Vec::new();
    for (angle, radius) in vertex_iter {
        let (sin, cos) = angle.sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        outer.push([x, y]);
        inner.push([x / 2., y / 2.]);
    }

    let outer = fj::Sketch::from_points(outer).with_color([0, 255, 0, 200]);
    let inner = fj::Sketch::from_points(inner);

    let footprint = fj::Difference2d::from_shapes(outer.into(), inner.into());

    let star = fj::Sweep::from_shape_and_length(footprint.into(), h);

    star.into()
}
