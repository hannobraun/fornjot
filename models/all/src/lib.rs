use fj::core::{
    math::{Scalar, Vector},
    operations::{build::BuildSolid, merge::Merge, transform::TransformObject},
    topology::Solid,
};

pub fn model(core: &mut fj::core::Core) -> Solid {
    // Just combine all the other models using offsets/rotations that won't
    // result in neat vertex positions or axis-aligned edges/faces. This is
    // useful for testing.

    let offset = Vector::from([5., 5., 5.]);
    let axis = Vector::from([1., 1., 1.]).normalize();
    let angle_rad = Scalar::PI / 6.;

    let models = [
        color::model(core),
        cuboid::model_old([1., 2., 3.], core),
        holes::model(0.5, core),
        spacer::model_old(2., 1., 1., core),
        split::model(1., 0.2, core),
        star::model(5, 2., 1., 1., core),
        vertices_indices::model(core),
    ];

    let mut all = Solid::empty();

    for (i, model) in models.into_iter().enumerate() {
        let f = i as f64;

        let model = model
            .translate(offset * f, core)
            .rotate(axis * angle_rad * f, core);

        all = all.merge(&model, core);
    }

    all
}
