use decorum::R32;

pub struct Sphere {
    pub radius: R32,
}

impl Sphere {
    pub fn from_radius(radius: impl Into<R32>) -> Self {
        Self {
            radius: radius.into(),
        }
    }
}
