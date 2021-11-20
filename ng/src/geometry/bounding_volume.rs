use crate::{
    geometry::vertices::Vertices as _,
    math::{Point, Vector},
};

/// Compute the bounding volume of a shape
///
/// The bounding volume is a volume that contains all of the shape.
pub trait BoundingVolume {
    /// Compute the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn aabb(&self) -> Aabb;
}

/// An axis-aligned bounding box
#[derive(Debug)]
pub struct Aabb {
    /// Minimum point of the axis-aligned bounding box
    pub min: Point,

    /// Maximum point of the axis-aligned bounding box
    pub max: Point,
}

impl Aabb {
    /// Create a bounding volume that encloses the provided vertices
    pub fn from_vertices(vertices: impl IntoIterator<Item = Point>) -> Self {
        let mut vertices = vertices.into_iter();

        // We need one vertex to seed our min/max coordinates, before going into
        // the loop. If the shape has no vertices, we'll just use the point at
        // the origin as a replacement. This will result in an empty bounding
        // box located at the origin.
        let vertex = vertices.next().unwrap_or(Point::origin());

        let mut min_x = vertex.x;
        let mut max_x = vertex.x;
        let mut min_y = vertex.y;
        let mut max_y = vertex.y;
        let mut min_z = vertex.z;
        let mut max_z = vertex.z;

        for vertex in vertices {
            if vertex.x < min_x {
                min_x = vertex.x;
            }
            if vertex.x > max_x {
                max_x = vertex.x;
            }
            if vertex.y < min_y {
                min_y = vertex.y;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
            if vertex.z < min_z {
                min_z = vertex.z;
            }
            if vertex.z > max_z {
                max_z = vertex.z;
            }
        }

        Self {
            min: [min_x, min_y, min_z].into(),
            max: [max_x, max_y, max_z].into(),
        }
    }

    /// Compute the size of the axis-aligned bounding box
    pub fn size(&self) -> Vector {
        self.max - self.min
    }
}

impl BoundingVolume for fj::Shape {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Shape2d(shape) => shape.aabb(),
            Self::Shape3d(shape) => shape.aabb(),
        }
    }
}

impl BoundingVolume for fj::Shape2d {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Circle(shape) => shape.aabb(),
            Self::Difference(shape) => shape.aabb(),
            Self::Square(shape) => shape.aabb(),
        }
    }
}

impl BoundingVolume for fj::Shape3d {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Sweep(shape) => shape.aabb(),
        }
    }
}

impl BoundingVolume for fj::Square {
    fn aabb(&self) -> Aabb {
        Aabb::from_vertices(self.vertices())
    }
}

impl BoundingVolume for fj::Sweep {
    fn aabb(&self) -> Aabb {
        let mut aabb = self.shape.aabb();
        aabb.max.z = self.length;
        aabb
    }
}
