use fj_core::interop::TriMesh;

use crate::{Args, Result};

/// # Process a model, given as a triangle mesh, according to the arguments
///
/// Will export the model, if the respective argument has been set. Will display
/// the model otherwise.
pub fn process_model(tri_mesh: TriMesh, args: Args) -> Result {
    if let Some(path) = args.export {
        crate::export::export(tri_mesh.external_triangles(), path)?;
    } else {
        crate::viewer::make_viewer_and_spawn_thread({
            let tri_mesh = tri_mesh.clone();

            |viewer| {
                crate::DEBUG_WINDOW.initialize(&viewer);
                viewer.open_window().display_mesh(tri_mesh);
            }
        })?;
    }

    Ok(())
}
