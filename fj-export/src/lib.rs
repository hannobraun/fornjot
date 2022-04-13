//! Exporting Fornjot models to external files

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
