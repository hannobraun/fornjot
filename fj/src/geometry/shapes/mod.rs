pub mod circle;
pub mod cylinder;
pub mod edge;
pub mod hypersphere;
pub mod parallelogram;
pub mod sphere;
pub mod toroid;
pub mod vertex;

pub use self::{
    circle::Circle, cylinder::Cylinder, edge::Edge, hypersphere::Hypersphere,
    parallelogram::Parallelogram, sphere::Sphere, toroid::Toroid,
    vertex::Vertex,
};
