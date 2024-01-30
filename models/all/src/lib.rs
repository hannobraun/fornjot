use fj::{
    core::{
        objects::Solid,
        operations::{
            build::BuildSolid, merge::Merge, transform::TransformObject,
        },
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

    let models = [
        color::model(services),
        cuboid::model([1., 2., 3.], services),
        holes::model(0.5, services),
        spacer::model(2., 1., 1., services),
        split::model(1., 0.2, services),
        star::model(5, 2., 1., 1., services),
    ];

    let mut all = Solid::empty();

    for (i, model) in models.into_iter().enumerate() {
        let f = i as f64;

        let model = model
            .translate(offset * f, services)
            .rotate(axis * angle_rad * f, services);

        all = all.merge(&model);
    }

    all
}
