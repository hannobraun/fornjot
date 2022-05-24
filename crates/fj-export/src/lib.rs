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

#![warn(missing_docs)]

use std::{fs::File, path::Path};

use thiserror::Error;

use fj_interop::mesh::Mesh;
use fj_math::{Point, Triangle};

/// Export the provided mesh to the file at the given path.
///
/// This function will create a file if it does not exist, and will truncate it if it does.
///
/// Currently 3MF & STL file types are supported. The case insensitive file extension of
/// the provided path is used to switch between supported types.
pub fn export(mesh: &Mesh<Point<3>>, path: &Path) -> Result<(), Error> {
    match path.extension() {
        Some(extension) if extension.to_ascii_uppercase() == "3MF" => {
            export_3mf(mesh, path)
        }
        Some(extension) if extension.to_ascii_uppercase() == "STL" => {
            export_stl(mesh, path)
        }
        Some(extension) => Err(Error::InvalidExtension(
            extension.to_string_lossy().into_owned(),
        )),
        None => Err(Error::NoExtension),
    }
}

fn export_3mf(mesh: &Mesh<Point<3>>, path: &Path) -> Result<(), Error> {
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

fn export_stl(mesh: &Mesh<Point<3>>, path: &Path) -> Result<(), Error> {
    let points = mesh
        .triangles()
        .map(|triangle| triangle.points)
        .collect::<Vec<_>>();

    let vertices = points.iter().map(|points| {
        points.map(|point| point.coords.components.map(|s| s.into_f32()))
    });

    let normals = points
        .iter()
        .map(|&points| points.into())
        .map(|triangle: Triangle<3>| triangle.normal())
        .map(|vector| vector.components.map(|s| s.into_f32()));

    let triangles = vertices
        .zip(normals)
        .map(|([v1, v2, v3], normal)| stl::Triangle {
            normal,
            v1,
            v2,
            v3,
            attr_byte_count: 0,
        })
        .collect::<Vec<_>>();

    let mut file = File::create(path)?;

    let binary_stl_file = stl::BinaryStlFile {
        header: stl::BinaryStlHeader {
            header: [0u8; 80],
            num_triangles: triangles
                .len()
                .try_into()
                .map_err(|_| Error::InvalidTriangleCount)?,
        },
        triangles,
    };

    stl::write_stl(&mut file, &binary_stl_file)?;

    Ok(())
}

/// An error that can occur while exporting
#[derive(Debug, Error)]
pub enum Error {
    /// No extension specified
    #[error("no extension specified")]
    NoExtension,

    /// Unrecognised extension found
    #[error("unrecognised extension found `{0:?}`")]
    InvalidExtension(String),

    /// I/O error whilst exporting to file
    #[error("I/O error whilst exporting to file")]
    Io(#[from] std::io::Error),

    /// Maximum triangle count exceeded
    #[error("maximum triangle count exceeded")]
    InvalidTriangleCount,

    /// Threemf error whilst exporting to 3MF file
    #[error("threemf error whilst exporting to 3MF file")]
    ThreeMF(#[from] threemf::Error),
}
