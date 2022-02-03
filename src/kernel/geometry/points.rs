// TASK: Un-suppress warning.
#![allow(unused)]

use super::Handle;

// TASK: Document.
pub struct Point1D {
    // TASK: Document.
    pub s: f64,

    // TASK: Document.
    pub handle: Handle<Point>,
}

// TASK: Document.
pub struct Point2D {
    // TASK: Document.
    pub u: f64,

    // TASK: Document.
    pub v: f64,

    // TASK: Document.
    pub handle: Handle<Point>,
}

// TASK: Document.
pub struct Point3D {
    // TASK: Document.
    pub x: f64,

    // TASK: Document.
    pub y: f64,

    // TASK: Document.
    pub z: f64,

    // TASK: Document.
    pub handle: Handle<Point>,
}

// TASK: Document.
pub struct Point;
