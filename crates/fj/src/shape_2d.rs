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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct PolyChain {
    segments: ffi_safe::Vec<SketchSegment>,
}

impl PolyChain {
    /// Construct an instance from a list of points
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        let points = points
            .into_iter()
            .map(|point| SketchSegment::LineTo { point })
            .collect();
        Self { segments: points }
    }

    /// Return the points that define the polygonal chain
    pub fn to_points(&self) -> Vec<SketchSegment> {
        self.segments.clone().into()
    }
}

/// A segment of a sketch
///
/// Each segment starts at the previous point of the sketch.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum SketchSegment {
    /// A line to a point
    LineTo {
        /// The destination point of the line
        point: [f64; 2],
    },
}
