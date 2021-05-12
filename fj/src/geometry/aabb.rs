use nalgebra::{Point, SVector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb<const D: usize> {
    pub min: Point<f32, D>,
    pub max: Point<f32, D>,
}

impl<const D: usize> Aabb<D> {
    pub fn from_min_and_size(
        min: Point<f32, D>,
        size: SVector<f32, D>,
    ) -> Self {
        assert!(size[0] >= 0.0 && size[1] >= 0.0 && size[2] >= 0.0);
        assert!(size.magnitude_squared() > 0.0);

        Self {
            min,
            max: min + size,
        }
    }

    pub fn size(&self) -> SVector<f32, D> {
        self.max - self.min
    }

    pub fn center(&self) -> Point<f32, D> {
        self.min + self.size() / 2.0
    }
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

    pub fn partition(&self) -> [Self; 8] {
        let size = self.size() / 2.0;

        let a: SVector<f32, 3> = [0.0, 0.0, 0.0].into();
        let b: SVector<f32, 3> = [0.0, 0.0, size.z].into();
        let c: SVector<f32, 3> = [0.0, size.y, 0.0].into();
        let d: SVector<f32, 3> = [0.0, size.y, size.z].into();
        let e: SVector<f32, 3> = [size.x, 0.0, 0.0].into();
        let f: SVector<f32, 3> = [size.x, 0.0, size.z].into();
        let g: SVector<f32, 3> = [size.x, size.y, 0.0].into();
        let h: SVector<f32, 3> = [size.x, size.y, size.z].into();

        [
            Self::from_min_and_size(self.min + a, size),
            Self::from_min_and_size(self.min + b, size),
            Self::from_min_and_size(self.min + c, size),
            Self::from_min_and_size(self.min + d, size),
            Self::from_min_and_size(self.min + e, size),
            Self::from_min_and_size(self.min + f, size),
            Self::from_min_and_size(self.min + g, size),
            Self::from_min_and_size(self.min + h, size),
        ]
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

    #[test]
    fn vertices_should_return_vertices() {
        let aabb = Aabb {
            min: [0.0, 0.0, 0.0].into(),
            max: [1.0, 1.0, 1.0].into(),
        };

        assert_eq!(
            aabb.vertices(),
            [
                [0.0, 0.0, 0.0].into(),
                [0.0, 0.0, 1.0].into(),
                [0.0, 1.0, 0.0].into(),
                [0.0, 1.0, 1.0].into(),
                [1.0, 0.0, 0.0].into(),
                [1.0, 0.0, 1.0].into(),
                [1.0, 1.0, 0.0].into(),
                [1.0, 1.0, 1.0].into(),
            ]
        )
    }
}
