use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    // TASK: Process arguments.
    dbg!(args);

    let outer_edge = fj::Circle { radius: 1.0 };
    let inner_edge = fj::Circle { radius: 0.5 };

    let footprint = fj::Difference {
        a: outer_edge.into(),
        b: inner_edge.into(),
    };

    let spacer = fj::Sweep {
        shape: footprint.into(),
        length: 1.0,
    };

    spacer.into()
}
