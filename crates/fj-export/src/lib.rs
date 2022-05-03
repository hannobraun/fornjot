//! # Fornjot Exporter
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! The purpose of this library is to export Fornjot models to external file
//! formats.
//!
//! [Fornjot]: https://www.fornjot.app/

#![deny(missing_docs)]

use std::path::Path;

use fj_interop::mesh::Mesh;
use fj_math::Point;

/// Export the provided mesh to the file at the given path
///
/// Currently only 3MF is supported as an export format. The file extension of
/// the provided path is ignored.
pub fn export(mesh: &Mesh<Point<3>>, path: &Path) -> Result<(), Error> {
    let vertices = mesh.vertices().map(|vertex| vertex.into()).collect();

    let indices: Vec<_> = mesh.indices().collect();
    let triangles = indices
        .chunks(3)
        .map(|triangle| {
            [
                triangle[0] as usize,
                triangle[1] as usize,
                triangle[2] as usize,
            ]
        })
        .collect();

    let mesh = threemf::TriangleMesh {
        vertices,
        triangles,
    };

    threemf::write(path, &mesh)?;

    Ok(())
}

pub use threemf::Error;
