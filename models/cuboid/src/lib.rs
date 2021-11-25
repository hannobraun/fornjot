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

    let rectangle = fj::Rectangle { size: width }.into();

    let cuboid = fj::Sweep {
        shape: rectangle,
        length: height,
    };

    cuboid.into()
}
