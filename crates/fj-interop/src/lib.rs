//! # Fornjot Interop Types
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! The purpose of this library is to define types that allow other components
//! of the Fornjot ecosystem to interoperate, without depending on each other.
//!
//! [Fornjot]: https://www.fornjot.app/

#![warn(missing_docs)]

pub mod debug;
pub mod mesh;
pub mod processed_shape;
pub mod status_report;
