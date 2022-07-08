#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};
use std::mem;
use std::sync::atomic;

use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Chain {
    /// The chain is a circle
    Circle(Circle),

    /// The chain is a polygonal chain
    PolyChain(PolyChain),
}

/// A circle
#[derive(Clone, Debug)]
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
#[derive(Debug)]
#[repr(C)]
pub struct PolyChain {
    // The fields are the raw parts of a `Vec`. `Sketch` needs to be FFI-safe,
    // meaning it can't store a `Vec` directly. It needs to take this detour.
    ptr: *mut [f64; 2],
    length: usize,
    capacity: usize,

    // The `Sketch` can be cloned, so we need to track the number of live
    // instances, so as to free the buffer behind `ptr` only when the last
    // one is dropped.
    rc: *mut atomic::AtomicUsize,
}

impl PolyChain {
    /// Construct an instance from a list of points
    pub fn from_points(mut points: Vec<[f64; 2]>) -> Self {
        // This can be cleaned up, once `Vec::into_raw_parts` is stable.
        let ptr = points.as_mut_ptr();
        let length = points.len();
        let capacity = points.capacity();

        // We're taking ownership of the memory here, so we can't allow `points`
        // to deallocate it.
        mem::forget(points);

        // Allocate the reference counter on the heap. It will be reclaimed
        // alongside `points` when it reaches 0.
        let rc = Box::new(atomic::AtomicUsize::new(1));
        let rc = Box::leak(rc) as *mut _;

        Self {
            ptr,
            length,
            capacity,
            rc,
        }
    }

    /// Return the points that define the polygonal chain
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
}

impl Clone for PolyChain {
    fn clone(&self) -> Self {
        // Increment the reference counter
        unsafe {
            (*self.rc).fetch_add(1, atomic::Ordering::AcqRel);
        }

        Self {
            ptr: self.ptr,
            length: self.length,
            capacity: self.capacity,
            rc: self.rc,
        }
    }
}

impl Drop for PolyChain {
    fn drop(&mut self) {
        // Decrement the reference counter
        let rc_last =
            unsafe { (*self.rc).fetch_sub(1, atomic::Ordering::AcqRel) };

        // If the value of the refcount before decrementing was 1,
        // then this must be the last Drop call. Reclaim all resources
        // allocated on the heap.
        if rc_last == 1 {
            unsafe {
                let points =
                    Vec::from_raw_parts(self.ptr, self.length, self.capacity);
                let rc = Box::from_raw(self.rc);

                drop(points);
                drop(rc);
            }
        }
    }
}

// `PolyChain` can be `Send`, because it encapsulates the raw pointer it
// contains, making sure memory ownership rules are observed.
unsafe impl Send for PolyChain {}

#[cfg(feature = "serde")]
impl ser::Serialize for PolyChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let serde_sketch = PolyChainSerde {
            points: self.to_points(),
        };

        serde_sketch.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for PolyChain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
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
#[derive(Serialize, Deserialize)]
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
            let rc =
                unsafe { (*poly_chain.rc).load(atomic::Ordering::Acquire) };
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
