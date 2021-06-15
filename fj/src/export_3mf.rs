use std::{fs::File, path::PathBuf};

use crate::Mesh;

/// Export mesh to 3MF file
///
/// See [3MF specification] and [Open Packaging Conventions Fundamentals].
///
/// [3MF specification]: https://3mf.io/specification/
/// [Open Packaging Conventions Fundamentals]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/opc/open-packaging-conventions-overview
pub fn export_3mf(_mesh: &Mesh, path: PathBuf) -> anyhow::Result<()> {
    let _file = File::create(path)?;

    // TASK: Export model to 3MF file.
    todo!()
}
