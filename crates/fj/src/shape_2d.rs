use abi_stable::std_types::{RArc, RBox, RVec};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Shape2d {
    /// A difference between two shapes
    Difference(RBox<Difference2d>),

    /// A sketch
    Sketch(Sketch),
}

impl Shape2d {
    /// Get the rendering color of the larger object in RGBA
    pub fn color(&self) -> [u8; 4] {
        match &self {
            Shape2d::Sketch(s) => s.color(),
            Shape2d::Difference(d) => d.color(),
        }
    }
}

/// A difference between two shapes
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Difference2d {
    shapes: [Shape2d; 2],
}

impl Difference2d {
    /// Create a `Difference2d` from two shapes
    pub fn from_shapes(shapes: [Shape2d; 2]) -> Self {
        Self { shapes }
    }

    /// Get the rendering color of the larger object in RGBA
    pub fn color(&self) -> [u8; 4] {
        self.shapes[0].color()
    }

    /// Access the shapes that make up the difference
    pub fn shapes(&self) -> &[Shape2d; 2] {
        &self.shapes
    }
}

impl From<Difference2d> for Shape {
    fn from(shape: Difference2d) -> Self {
        Self::Shape2d(shape.into())
    }
}

impl From<Difference2d> for Shape2d {
    fn from(shape: Difference2d) -> Self {
        Self::Difference(RBox::new(shape))
    }
}

/// A sketch
///
/// Sketches are currently limited to a single cycle of straight lines,
/// represented by a number of points. For example, if the points a, b, and c
/// are provided, the edges ab, bc, and ca are assumed.
///
/// Nothing about these edges is checked right now, but algorithms might assume
/// that the edges are non-overlapping. If you create a `Sketch` with
/// overlapping edges, you're on your own.
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sketch {
    chain: Chain,

    // The color of the sketch in RGBA
    color: [u8; 4],
}

impl Sketch {
    /// Create a sketch from a bunch of points
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        Self {
            chain: Chain::PolyChain(PolyChain::from_points(points)),
            color: [255, 0, 0, 255],
        }
    }

    /// Create a sketch from a circle
    pub fn from_circle(circle: Circle) -> Self {
        Self {
            chain: Chain::Circle(circle),
            color: [255, 0, 0, 255],
        }
    }

    /// Set the rendering color of the sketch in RGBA
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }

    /// Access the chain of the sketch
    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Get the rendering color of the sketch in RGBA
    pub fn color(&self) -> [u8; 4] {
        self.color
    }
}

/// A chain of elements that is part of a [`Sketch`]
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Chain {
    /// The chain is a circle
    Circle(Circle),

    /// The chain is a polygonal chain
    PolyChain(PolyChain),
}

/// A circle that is part of a [`Sketch`]
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    radius: f64,
}

impl Circle {
    /// Construct a new circle with a specific radius
    pub fn from_radius(radius: f64) -> Self {
        Self { radius }
    }

    /// Access the circle's radius
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

/// A polygonal chain that is part of a [`Sketch`]
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolyChain(RArc<RVec<[f64; 2]>>);

impl PolyChain {
    /// Construct an instance from a list of points
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        PolyChain(RArc::new(points.into()))
    }

    /// Return the points that define the polygonal chain
    pub fn to_points(&self) -> Vec<[f64; 2]> {
        self.0.to_vec()
    }
}

impl From<Sketch> for Shape {
    fn from(shape: Sketch) -> Self {
        Self::Shape2d(shape.into())
    }
}

impl From<Sketch> for Shape2d {
    fn from(shape: Sketch) -> Self {
        Shape2d::Sketch(shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_points() -> Vec<[f64; 2]> {
        vec![[1.0, 1.0], [2.0, 1.0], [2.0, 2.0], [1.0, 2.0]]
    }

    #[test]
    fn test_poly_chain_preserve_points() {
        let points = test_points();
        let poly_chain = PolyChain::from_points(points.clone());

        assert_eq!(poly_chain.to_points(), points);
    }

    #[test]
    fn test_poly_chain_rc() {
        let assert_rc = |poly_chain: &PolyChain, expected_rc: usize| {
            let rc = RArc::strong_count(&poly_chain.0);
            assert_eq!(
                rc, expected_rc,
                "Sketch has rc = {rc}, expected {expected_rc}"
            );
        };

        let poly_chain = PolyChain::from_points(test_points());
        assert_rc(&poly_chain, 1);

        let (s2, s3) = (poly_chain.clone(), poly_chain.clone());
        assert_rc(&poly_chain, 3);

        drop(s2);
        assert_rc(&poly_chain, 2);

        drop(s3);
        assert_rc(&poly_chain, 1);

        // rc is deallocated after the last drop, so we can't assert that it's 0
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_poly_chain_serialize_loopback() {
        use serde_json::{from_str, to_string};

        let poly_chain = PolyChain::from_points(test_points());

        let json = to_string(&poly_chain).expect("failed to serialize sketch");
        let poly_chain_de: PolyChain =
            from_str(&json).expect("failed to deserialize sketch");

        // ensure same content
        assert_eq!(poly_chain.to_points(), poly_chain_de.to_points());
    }
}
