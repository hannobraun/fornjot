use fj_core::{
    interop::{Color, MeshTriangle, TriMesh},
    new::Model,
};

use crate::{Arguments, Result};

/// # Process a model, according to the provided arguments
///
/// Exports the model, if the respective argument has been set. Display the
/// model otherwise.
pub fn process_model(model: Model, args: Arguments) -> Result {
    let triangles = model.topology.solids[model.solid]
        .boundary
        .iter()
        .flat_map(|&face| &model.topology.faces[face].approx);

    let mut tri_mesh = TriMesh::new();

    for &triangle in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: triangle,
            is_internal: false,
            color: Color::default(),
        });
    }

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
