use crate::{abi::ffi_safe, Shape};

/// A 2-dimensional shape
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum Shape2d {
    /// A difference between two shapes
    Difference(Box<Difference2d>),

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
///
/// # Examples
///
/// Convenient syntax for this operation is available through [`crate::syntax`].
///
/// ``` rust
/// # let a = fj::Sketch::from_points(vec![[0., 0.], [1., 0.], [0., 1.]]);
/// # let b = fj::Sketch::from_points(vec![[2., 0.], [3., 0.], [2., 1.]]);
/// use fj::syntax::*;
///
/// // `a` and `b` can be anything that converts to `fj::Shape2d`
/// let difference = a.difference(&b);
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        Self::Difference(Box::new(shape))
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
///
/// # Examples
///
/// Convenient syntax for this operation is available through [`crate::syntax`].
///
/// ``` rust
/// use fj::syntax::*;
///
/// // `a` and `b` can be anything that converts to `fj::Shape`
/// let sketch = [[0., 0.], [1., 0.], [0., 1.]].sketch();
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Sketch {
    chain: Chain,
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
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum Chain {
    /// The chain is a circle
    Circle(Circle),

    /// The chain is a polygonal chain
    PolyChain(PolyChain),
}

/// A circle that is part of a [`Sketch`]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct PolyChain {
    points: ffi_safe::Vec<[f64; 2]>,
}

impl PolyChain {
    /// Construct an instance from a list of points
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        let points = points.into();
        Self { points }
    }

    /// Return the points that define the polygonal chain
    pub fn to_points(&self) -> Vec<[f64; 2]> {
        self.points.clone().into()
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for PolyChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let serde_sketch = PolyChainSerde {
            points: self.to_points(),
        };

        serde_sketch.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for PolyChain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        PolyChainSerde::deserialize(deserializer)
            .map(|serde_sketch| PolyChain::from_points(serde_sketch.points))
    }
}

/// An owned, non-repr-C [`PolyChain`]
///
/// De/serializing a non-trivial structure with raw pointers is a hassle.
/// This structure is a simple, owned intermediate form that can use the derive
/// macros provided by serde. The implementation of the `Serialize` and
/// `Deserialize` traits for [`PolyChain`] use this type as a stepping stone.
///
/// Note that constructing this requires cloning the points behind
/// [`PolyChain`]. If de/serialization turns out to be a bottleneck, a more
/// complete implementation will be required.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Polyline")]
struct PolyChainSerde {
    points: Vec<[f64; 2]>,
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
    #[cfg(feature = "serde")]
    #[test]
    fn test_poly_chain_serialize_loopback() {
        use serde_json::{from_str, to_string};

        let poly_chain = super::PolyChain::from_points(vec![
            [1.0, 1.0],
            [2.0, 1.0],
            [2.0, 2.0],
            [1.0, 2.0],
        ]);

        let json = to_string(&poly_chain).expect("failed to serialize sketch");
        let poly_chain_de: super::PolyChain =
            from_str(&json).expect("failed to deserialize sketch");

        // ensure same content
        assert_eq!(poly_chain.to_points(), poly_chain_de.to_points());
    }
}
