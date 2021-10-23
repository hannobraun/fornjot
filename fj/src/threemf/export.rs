use std::{
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

use thiserror::Error;

use zip::{result::ZipError, write::FileOptions, ZipWriter};

use crate::geometry::shapes::Mesh;

/// Export a triangle mesh to a 3MF file
///
/// See [3MF specification] and [Open Packaging Conventions].
///
/// [3MF specification]: https://3mf.io/specification/
/// [Open Packaging Conventions]: https://standards.iso.org/ittf/PubliclyAvailableStandards/c061796_ISO_IEC_29500-2_2012.zip
pub fn export(mesh: &Mesh<3>, path: PathBuf) -> Result<(), Error> {
    let file = File::create(&path)?;
    let mut archive = ZipWriter::new(file);

    archive.start_file("[Content_Types].xml", FileOptions::default())?;
    archive.write_all(include_bytes!("content-types.xml"))?;

    archive.start_file("_rels/.rels", FileOptions::default())?;
    archive.write_all(include_bytes!("rels.xml"))?;

    archive.start_file("3D/model.model", FileOptions::default())?;
    write_mesh(mesh, &mut archive)?;

    archive.finish()?;

    Ok(())
}

fn write_mesh(mesh: &Mesh<3>, mut sink: impl Write) -> io::Result<()> {
    sink.write_all(include_bytes!("model-header.xml"))?;

    writeln!(sink, "\t\t\t\t<vertices>")?;
    for vertex in mesh.vertices() {
        writeln!(
            sink,
            "\t\t\t\t\t<vertex x=\"{}\" y=\"{}\" z=\"{}\" />",
            vertex.x, vertex.y, vertex.z,
        )?;
    }
    writeln!(sink, "\t\t\t\t</vertices>")?;

    writeln!(sink, "\t\t\t\t<triangles>")?;
    for [i1, i2, i3] in mesh.triangle_indices() {
        writeln!(
            sink,
            "\t\t\t\t\t<triangle v1=\"{}\" v2=\"{}\" v3=\"{}\" />",
            i1, i2, i3,
        )?;
    }
    writeln!(sink, "\t\t\t\t</triangles>")?;

    sink.write_all(include_bytes!("model-footer.xml"))?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Zip error")]
    Zip(#[from] ZipError),
}
