use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    // TASK: Process arguments.
    dbg!(args);

    let square = fj::Square { size: 1.0 }.into();

    let cube = fj::Sweep {
        shape: square,
        length: 1.0,
    };

    cube.into()
}
