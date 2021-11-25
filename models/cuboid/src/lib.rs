use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let x = args.get("x").unwrap_or(&"3.0".to_owned()).parse().unwrap();
    let y = args.get("y").unwrap_or(&"2.0".to_owned()).parse().unwrap();
    let z = args.get("z").unwrap_or(&"1.0".to_owned()).parse().unwrap();

    let rectangle = fj::Rectangle { x, y }.into();

    let cuboid = fj::Sweep {
        shape: rectangle,
        length: z,
    };

    cuboid.into()
}
