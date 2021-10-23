//! Support for 3MF (3D Manufacturing Format) for the Rust programming language
//!
//! This library provides support for 3MF files to programs written in the Rust
//! programming language. So far, functionality is limited to writing 3MF files,
//! and only the most basic features of 3MF are supported.
//!
//! Adding support for reading 3MF files, and for more features of the 3MF
//! format is very desirable, and any contributions toward that are very
//! welcome.
//!
//! # Further Reading
//!
//! See [3MF specification] and [Open Packaging Conventions].
//!
//! [3MF specification]: https://3mf.io/specification/
//! [Open Packaging Conventions]: https://standards.iso.org/ittf/PubliclyAvailableStandards/c061796_ISO_IEC_29500-2_2012.zip

pub mod export;
pub mod mesh;

pub use self::{export::export, mesh::TriangleMesh};
