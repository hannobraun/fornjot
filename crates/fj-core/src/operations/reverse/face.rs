use crate::{objects::Face, operations::Insert, services::Services};

use super::Reverse;

impl Reverse for Face {
    fn reverse(&self, services: &mut Services) -> Self {
        let region = self.region().reverse(services).insert(services);
        Face::new(self.surface().clone(), region)
    }
}
