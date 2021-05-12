use nalgebra::Point;

#[derive(Clone, Copy, Debug)]
pub struct Aabb<const D: usize> {
    pub min: Point<f32, D>,
    pub max: Point<f32, D>,
}

impl Aabb<2> {
    pub fn extend(self, min: f32, max: f32) -> Aabb<3> {
        Aabb {
            min: [self.min.x, self.min.y, min].into(),
            max: [self.max.x, self.max.y, max].into(),
        }
    }
}

impl Aabb<3> {
    pub fn vertices(&self) -> [Point<f32, 3>; 8] {
        [
            [self.min.x, self.min.y, self.min.z].into(),
            [self.min.x, self.min.y, self.max.z].into(),
            [self.min.x, self.max.y, self.min.z].into(),
            [self.min.x, self.max.y, self.max.z].into(),
            [self.max.x, self.min.y, self.min.z].into(),
            [self.max.x, self.min.y, self.max.z].into(),
            [self.max.x, self.max.y, self.min.z].into(),
            [self.max.x, self.max.y, self.max.z].into(),
        ]
    }
}

impl<const D: usize> Aabb<D> {
    pub fn center(&self) -> Point<f32, D> {
        self.min + (self.max - self.min) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;

    #[test]
    fn center_should_return_center() {
        let aabb = Aabb {
            min: [1.0, 2.0].into(),
            max: [3.0, 4.0].into(),
        };

        assert_eq!(aabb.center(), [2.0, 3.0].into());
    }
}
