use crate::objects::Face;

use super::Reverse;

impl Reverse for Face {
    fn reverse(self) -> Self {
        let exterior = self.exterior().clone().reverse();
        let interiors = self.interiors().map(|cycle| cycle.clone().reverse());

        Face::new(exterior)
            .with_interiors(interiors)
            .with_color(self.color())
    }
}
