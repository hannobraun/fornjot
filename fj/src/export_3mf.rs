use std::{fs::File, path::PathBuf};

use crate::Mesh;

pub fn export_3mf(_mesh: &Mesh, path: PathBuf) -> anyhow::Result<()> {
    let _file = File::create(path)?;

    // TASK: Export model to 3MF file.
    todo!()
}
