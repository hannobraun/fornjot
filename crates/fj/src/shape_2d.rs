use std::mem;

use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A difference between two shapes
    Difference(Box<Difference2d>),

    /// A sketch
    Sketch(Sketch),
}

impl Shape2d {
    /// Get the rendering color of the larger object in RGBA
    pub fn color(&self) -> [u8; 4] {
        match &self {
            Shape2d::Circle(c) => c.color(),
            Shape2d::Sketch(s) => s.color(),
            Shape2d::Difference(d) => d.color(),
        }
    }
}

/// A circle
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    radius: f64,
    // The color of the circle in RGBA
    color: [u8; 4],
}

impl Circle {
    /// Construct a new circle with a specific radius
    pub fn from_radius(radius: f64) -> Self {
        Self {
            radius,
            color: [255, 0, 0, 255],
        }
    }

    /// Access the circle's radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set the rendering color of the circle in RGBA
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }

    /// Set the rendering color of the circle in RGBA
    pub fn set_color(&mut self, color: [u8; 4]) {
        self.color = color;
    }

    /// Get the rendering color of the circle in RGBA
    pub fn color(&self) -> [u8; 4] {
        self.color
    }
}

impl From<Circle> for Shape {
    fn from(shape: Circle) -> Self {
        Self::Shape2d(shape.into())
    }
}

impl From<Circle> for Shape2d {
    fn from(shape: Circle) -> Self {
        Self::Circle(shape)
    }
}

/// A difference between two shapes
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sketch {
    // The fields are the raw parts of a `Vec`. `Sketch` needs to be FFI-safe,
    // meaning it can't store a `Vec` directly. It needs to take this detour.
    ptr: *mut [f64; 2],
    length: usize,
    capacity: usize,
    // The color of the sketch in RGBA
    color: [u8; 4],
}

impl Sketch {
    /// Create a sketch from a bunch of points
    pub fn from_points(mut points: Vec<[f64; 2]>) -> Self {
        // This can be cleaned up, once `Vec::into_raw_parts` is stable.
        let ptr = points.as_mut_ptr();
        let length = points.len();
        let capacity = points.capacity();

        // We're taking ownership of the memory here, so we can't allow `points`
        // to deallocate it.
        mem::forget(points);

        Self {
            ptr,
            length,
            capacity,
            color: [255, 0, 0, 255],
        }
    }

    /// Return the points of the sketch
    pub fn to_points(&self) -> Vec<[f64; 2]> {
        // This is sound. All invariants are automatically kept, as the raw
        // parts come from an original `Vec` that is identical to the new one we
        // create here, and aren't being modified anywhere.
        let points = unsafe {
            Vec::from_raw_parts(self.ptr, self.length, self.capacity)
        };

        // Ownership of the pointer in `self.raw_parts` transferred to `points`.
        // We work around that, by returning a clone of `points` (hence not
        // giving ownership to the caller).
        let ret = points.clone();

        // Now we just need to forget that `points` ever existed, and we keep
        // ownership of the pointer.
        mem::forget(points);

        ret
    }

    /// Set the rendering color of the sketch in RGBA
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }

    /// Set the rendering color of the sketch in RGBA
    pub fn set_color(&mut self, color: [u8; 4]) {
        self.color = color;
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

// `Sketch` can be `Send`, because it encapsulates the raw pointer it contains,
// making sure memory ownership rules are observed.
unsafe impl Send for Sketch {}
