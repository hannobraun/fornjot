use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let width = args
        .get("width")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();
    let height = args
        .get("height")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();

    let square = fj::Square { size: width }.into();

    let cube = fj::Sweep {
        shape: square,
        length: height,
    };

    cube.into()
}
