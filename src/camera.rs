use nalgebra::{TAffine, Transform, Translation};

/// The camera abstraction
///
/// Please note that the metaphor we're using (which influences how mouse input
/// is handled, for example) is not that of a camera freely flying through a
/// static scene. Instead, the camera is static, and the model is freely
/// translated and rotated.
#[derive(Debug)]
pub struct Camera {
    /// The rotational part of the transform
    ///
    /// This is not an `nalgebra::Rotation`, as rotations happen around a center
    /// point, which means they must include a translational component.
    pub rotation: Transform<f64, TAffine, 3>,

    pub translation: Translation<f64, 2>,
    pub distance: f64,
}

impl Camera {
    pub fn new(initial_distance: f64) -> Self {
        Self {
            rotation: Transform::identity(),
            translation: Translation::identity(),
            distance: initial_distance,
        }
    }

    pub fn near_plane(&self) -> f64 {
        0.1
    }

    pub fn far_plane(&self) -> f64 {
        1000.0
    }

    pub fn view_transform(&self) -> Transform<f64, TAffine, 3> {
        // Using a mutable variable cleanly takes care of any type inference
        // problems that this operation would otherwise have.
        let mut transform = Transform::identity();

        transform *= Translation::from([
            self.translation.x,
            self.translation.y,
            -self.distance,
        ]);
        transform *= self.rotation;

        transform
    }
}
