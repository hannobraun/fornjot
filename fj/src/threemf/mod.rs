use std::{fs::File, io, path::PathBuf};

use thiserror::Error;

use tracing::info;

use zip::{result::ZipError, write::FileOptions, ZipWriter};

use crate::Mesh;

/// Export mesh to 3MF file
///
/// See [3MF specification] and [Open Packaging Conventions].
///
/// [3MF specification]: https://3mf.io/specification/
/// [Open Packaging Conventions]: https://standards.iso.org/ittf/PubliclyAvailableStandards/c061796_ISO_IEC_29500-2_2012.zip
pub fn export(_mesh: &Mesh, path: PathBuf) -> Result<(), Error> {
    let name = path
        .file_stem()
        .ok_or_else(|| Error::NoFileName(path.clone()))?
        .to_string_lossy();

    info!("Exporting \"{}\" to `{}`", name, path.display());

    let file = File::create(&path)?;

    let mut archive = ZipWriter::new(file);
    archive.start_file(format!("3D/{}.model", name), FileOptions::default())?;
    archive.finish()?;

    // TASK: Export model to 3MF file.
    todo!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected path to file, got `{0}`")]
    NoFileName(PathBuf),

    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Zip error")]
    Zip(#[from] ZipError),
}
