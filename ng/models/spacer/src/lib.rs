#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let outer_edge = fj::Circle { radius: 1.0 };

    // TASK: Make hole in circle.
    let footprint = outer_edge;

    let spacer = fj::Sweep {
        shape: footprint.into(),
        length: 1.0,
    };

    spacer.into()
}
