use fj::syntax::*;

#[fj::model]
pub fn model(
    #[value(default = 1.0, min = inner * 1.01)] outer: f64,
    #[value(default = 0.5, max = outer * 0.99)] inner: f64,
    #[value(default = 1.0)] height: f64,
) -> fj::Shape {
    let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
    let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}
