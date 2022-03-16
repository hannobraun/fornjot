use std::collections::HashMap;

use fj::prelude::*;

#[no_mangle]
pub extern "C" fn model(_: &HashMap<String, String>) -> fj::Shape {
    #[rustfmt::skip]
    let vertices = vec![
        [-0.5, -0.5],
        [ 0.5, -0.5],
        [ 0.5,  0.5],
        [-0.5,  0.5],
    ];

    let cube_a = fj::Sketch::from_points(vertices).sweep(1.0);
    let cube_b = cube_a.clone().translate([1.5, 0., 0.5]);

    let disjoint_union = cube_a.group(&cube_b);

    disjoint_union.into()
}
