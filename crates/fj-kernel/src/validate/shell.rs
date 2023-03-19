use std::collections::HashMap;

use crate::{objects::Shell, storage::ObjectId};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        ShellValidationError::validate_watertight(self, config, errors);
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {
    /// [`Shell`] contains global_edges not referred to by two half_edges
    #[error("Shell is not watertight")]
    NotWaterTight,
}

impl ShellValidationError {
    fn validate_watertight(
        shell: &Shell,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let faces = shell.faces();
        let mut half_edge_to_faces: HashMap<ObjectId, usize> = HashMap::new();
        for face in faces {
            let cycles =
                face.interiors().chain(std::iter::once(face.exterior()));
            for cycle in cycles {
                for half_edge in cycle.half_edges() {
                    let id = half_edge.global_form().id();
                    let entry = half_edge_to_faces.entry(id);
                    *entry.or_insert(0) += 1;
                }
            }
        }

        // Each global edge should have exactly two half edges that are part of the shell
        if half_edge_to_faces.iter().find(|(_, c)| **c != 2).is_some() {
            errors.push(Self::NotWaterTight.into())
        }
    }
}
