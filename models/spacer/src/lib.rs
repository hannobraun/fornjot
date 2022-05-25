use std::collections::HashMap;

use fj::syntax::*;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let outer = args
        .get("outer")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .expect("Could not parse parameter `outer`");
    let inner = args
        .get("inner")
        .unwrap_or(&"0.5".to_owned())
        .parse()
        .expect("Could not parse parameter `inner`");
    let height: f64 = args
        .get("height")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .expect("Could not parse parameter `height`");

    let outer_edge = fj::Circle::from_radius(outer);
    let inner_edge = fj::Circle::from_radius(inner);

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}
