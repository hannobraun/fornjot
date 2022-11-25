use crate::{
    insert::Insert,
    objects::{Face, Objects},
    partial::HasPartial,
    services::Service,
    storage::Handle,
    validate::ValidationError,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(
        self,
        objects: &mut Service<Objects>,
    ) -> Result<Self, ValidationError> {
        let exterior = self.exterior().clone().reverse(objects)?;
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().reverse(objects))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Face::partial()
            .with_exterior(exterior)
            .with_interiors(interiors)
            .with_color(self.color())
            .build(objects)
            .insert(objects))
    }
}
