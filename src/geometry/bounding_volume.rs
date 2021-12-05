use crate::math::{Point, Vector};

/// An axis-aligned bounding box
#[derive(Debug)]
pub struct Aabb {
    /// Minimum point of the axis-aligned bounding box
    pub mins: Point,

    /// Maximum point of the axis-aligned bounding box
    pub maxs: Point,
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
            mins: [min_x, min_y, min_z].into(),
            maxs: [max_x, max_y, max_z].into(),
        }
    }

    /// Compute the size of the axis-aligned bounding box
    pub fn size(&self) -> Vector {
        self.maxs - self.mins
    }
}
