use fj::{
    core::{
        objects::Solid,
        operations::{merge::Merge, transform::TransformObject},
        services::Services,
    },
    math::{Scalar, Vector},
};

pub fn model(services: &mut Services) -> Solid {
    // Just combine all the other models using offsets/rotations that won't
    // result in neat vertex positions or axis-aligned edges/faces. This is
    // useful for testing.

    let offset = Vector::from([5., 5., 5.]);
    let axis = Vector::from([1., 1., 1.]).normalize();
    let angle_rad = Scalar::PI / 6.;

    let cuboid = cuboid::model([1., 2., 3.], services)
        .translate(offset * 1., services)
        .rotate(axis * angle_rad * 1., services);
    let spacer = spacer::model(2., 1., 1., services)
        .translate(offset * 2., services)
        .rotate(axis * angle_rad * 2., services);
    let star = star::model(5, 2., 1., 1., services)
        .translate(offset * 3., services)
        .rotate(axis * angle_rad * 3., services);
    let split = split::model(1., 0.2, services)
        .translate(offset * 4., services)
        .rotate(axis * angle_rad * 4., services);
    let holes = holes::model(0.5, services)
        .translate(offset * 5., services)
        .rotate(axis * angle_rad * 5., services);

    cuboid
        .merge(&spacer)
        .merge(&star)
        .merge(&split)
        .merge(&holes)
}
