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

use std::{fs::File, path::Path};

use anyhow::{anyhow, Result};

use fj_interop::mesh::Mesh;
use fj_math::{Point, Triangle, Vector};

/// Export the provided mesh to the file at the given path.
///
/// This function will create a file if it does not exist, and will truncate it if it does.
///
/// Currently 3MF & STL file types are supported. The case insensitive file extension of
/// the provided path is used to switch between supported types.
pub fn export(mesh: &Mesh<Point<3>>, path: &Path) -> Result<()> {
    match path.extension() {
        Some(extension) if extension.to_ascii_uppercase() == "3MF" => {
            export_3mf(mesh, path)
        }
        Some(extension) if extension.to_ascii_uppercase() == "STL" => {
            export_stl(mesh, path)
        }
        Some(extension) => {
            Err(anyhow!("Extension not recognised, got {:?}", extension))
        }
        None => Err(anyhow!("No extension specified")),
    }
}

fn export_3mf(mesh: &Mesh<Point<3>>, path: &Path) -> Result<()> {
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

fn export_stl(mesh: &Mesh<Point<3>>, path: &Path) -> Result<()> {
    let points = mesh
        .triangles()
        .map(|triangle| triangle.points)
        .collect::<Vec<_>>();

    let vertices = points.iter().map(|points| {
        points.map(|point| {
            stl_io::Vertex::new([
                point.x.into_f32(),
                point.y.into_f32(),
                point.z.into_f32(),
            ])
        })
    });

    let normals = mesh
        .triangles()
        .map(|triangle| triangle.points.into())
        .map(|triangle: Triangle<3>| triangle.to_parry().normal())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("Unable to compute normal"))?;

    let normals = normals.iter().map(|vector| vector.into_inner().into()).map(
        |vector: Vector<3>| {
            stl_io::Normal::new([
                vector.x.into_f32(),
                vector.y.into_f32(),
                vector.z.into_f32(),
            ])
        },
    );

    let mesh = vertices
        .zip(normals)
        .map(|(vertices, normal)| stl_io::Triangle { normal, vertices });

    let mut file = File::create(path)?;

    stl_io::write_stl(&mut file, mesh).unwrap();

    Ok(())
}
