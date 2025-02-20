use crate::{
    Core,
    operations::insert::Insert,
    storage::Handle,
    topology::{Curve, Vertex},
};

use super::SweepCache;

/// # Sweep a [`Vertex`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepVertex: Sized {
    /// # Sweep the vertex
    ///
    /// Returns the curve that the vertex was swept along, as well as a new
    /// vertex to represent the point at the end of the sweep.
    ///
    ///
    /// ## Comparison to Other Sweep Operations
    ///
    /// This method is a bit weird, compared to most other sweep operations, in
    /// that it doesn't actually do any sweeping. That is because because both
    /// [`Vertex`] and [`Curve`] do not define any geometry (please refer to
    /// their respective documentation). Because of that, this method doesn't
    /// even take the sweep path as an argument.
    ///
    /// The reason this code still exists as part of the sweep infrastructure,
    /// is to make sure that sweeping the same vertex multiple times always
    /// results in the same curve. This is also the reason that this trait is
    /// only implemented for `Handle<Vertex>` and produces a `Handle<Curve>`.
    fn sweep_vertex(
        &self,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> (Handle<Curve>, Handle<Vertex>);
}

impl SweepVertex for Handle<Vertex> {
    fn sweep_vertex(
        &self,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> (Handle<Curve>, Handle<Vertex>) {
        let curve = cache
            .curves
            .entry(self.id())
            .or_insert_with(|| Curve::new().insert(core))
            .clone();

        let vertex = cache
            .vertices
            .entry(self.id())
            .or_insert_with(|| Vertex::new().insert(core))
            .clone();

        (curve, vertex)
    }
}
