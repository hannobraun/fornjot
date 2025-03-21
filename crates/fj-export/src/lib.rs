//! # Fornjot Exporter
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library exports Fornjot models to external file formats.
//!
//! [Fornjot]: https://www.fornjot.app/

use std::{
    fs::File,
    io::{Seek, Write},
    path::Path,
};

use thiserror::Error;

use fj_interop::{TriMesh, vertices_to_indexed_vertices};
use fj_math::Triangle;

/// # Export the provided mesh to the file at the given path
///
/// This function will create a file if it does not exist, and will truncate it
/// if it does.
///
/// Currently 3MF & STL file types are supported. The case insensitive file
/// extension of the provided path is used to switch between supported types.
pub fn export(tri_mesh: &TriMesh, path: impl AsRef<Path>) -> Result<(), Error> {
    match path.as_ref().extension() {
        Some(extension) if extension.eq_ignore_ascii_case("3MF") => {
            let mut file = File::create(path)?;
            export_3mf(tri_mesh, &mut file)
        }
        Some(extension) if extension.eq_ignore_ascii_case("STL") => {
            let mut file = File::create(path)?;
            export_stl(tri_mesh, &mut file)
        }
        Some(extension) if extension.eq_ignore_ascii_case("OBJ") => {
            let mut file = File::create(path)?;
            export_obj(tri_mesh, &mut file)
        }
        Some(extension) => Err(Error::InvalidExtension(
            extension.to_string_lossy().into_owned(),
        )),
        None => Err(Error::NoExtension),
    }
}

/// # Export the provided mesh to the provided writer in the 3MF format
pub fn export_3mf(
    tri_mesh: &TriMesh,
    write: impl Write + Seek,
) -> Result<(), Error> {
    let (vertices, indices) = vertices_to_indexed_vertices(
        tri_mesh
            .triangles
            .iter()
            .flat_map(|triangle| triangle.inner.points),
        |point| threemf::model::Vertex {
            x: point.x.into_f64(),
            y: point.y.into_f64(),
            z: point.z.into_f64(),
        },
    );

    let triangles = indices
        .chunks(3)
        .map(|triangle| threemf::model::Triangle {
            v1: triangle[0] as usize,
            v2: triangle[1] as usize,
            v3: triangle[2] as usize,
        })
        .collect();

    let mesh = threemf::Mesh {
        vertices: threemf::model::Vertices { vertex: vertices },
        triangles: threemf::model::Triangles {
            triangle: triangles,
        },
    };

    threemf::write(write, mesh)?;

    Ok(())
}

/// # Export the provided mesh to the provided writer in the STL format
pub fn export_stl(
    tri_mesh: &TriMesh,
    mut write: impl Write,
) -> Result<(), Error> {
    let points = tri_mesh
        .triangles
        .iter()
        .map(|triangle| triangle.inner.points)
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

    stl::write_stl(&mut write, &binary_stl_file)?;

    Ok(())
}

/// # Export the provided mesh to the provided writer in the OBJ format
pub fn export_obj(
    tri_mesh: &TriMesh,
    mut write: impl Write,
) -> Result<(), Error> {
    for (cnt, t) in tri_mesh.triangles.iter().enumerate() {
        // write each point of the triangle
        for v in t.inner.points {
            wavefront_rs::obj::writer::Writer { auto_newline: true }
                .write(
                    &mut write,
                    &wavefront_rs::obj::entity::Entity::Vertex {
                        x: v.x.into_f64(),
                        y: v.y.into_f64(),
                        z: v.z.into_f64(),
                        w: None,
                    },
                )
                .or(Err(Error::OBJ))?;
        }

        // write the triangle
        wavefront_rs::obj::writer::Writer { auto_newline: true }
            .write(
                &mut write,
                &wavefront_rs::obj::entity::Entity::Face {
                    vertices: vec![
                        wavefront_rs::obj::entity::FaceVertex {
                            vertex: (cnt * 3 + 1) as i64,
                            texture: None,
                            normal: None,
                        },
                        wavefront_rs::obj::entity::FaceVertex {
                            vertex: (cnt * 3 + 2) as i64,
                            texture: None,
                            normal: None,
                        },
                        wavefront_rs::obj::entity::FaceVertex {
                            vertex: (cnt * 3 + 3) as i64,
                            texture: None,
                            normal: None,
                        },
                    ],
                },
            )
            .or(Err(Error::OBJ))?;
    }

    Ok(())
}

/// An error that can occur while exporting
#[derive(Debug, Error)]
pub enum Error {
    /// No extension specified
    #[error("no extension specified")]
    NoExtension,

    /// Unrecognized extension found
    #[error("unrecognized extension found `{0:?}`")]
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

    /// OBJ exporter error whilst exporting to OBJ file
    #[error("obj error whilst exporting to OBJ file")]
    OBJ,
}
