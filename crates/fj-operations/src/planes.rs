use fj_kernel::{
    objects::Surface,
    stores::{Handle, Stores},
};

/// The static planes
///
/// Keeps [`Handle`]s to the xy-, xz- and yz-planes. The purpose of this struct
/// is to provide these handles to implementations of [`Shape`], so they don't
/// have to create a duplicate `Surface` whenever they need one of those.
///
/// [`Shape`]: crate::Shape
pub struct Planes {
    xy: Handle<Surface>,
    xz: Handle<Surface>,
    yz: Handle<Surface>,
}

impl Planes {
    /// Create a new instance of `Planes`
    ///
    /// Please note that the whole point of this struct is to not duplicate the
    /// standard planes, and creating multiple instances of it defeats that
    /// point.
    ///
    /// Create one instance of this struct, then share it everywhere it's
    /// needed.
    pub fn new(stores: &Stores) -> Self {
        let xy = stores.surfaces.insert(Surface::xy_plane());
        let xz = stores.surfaces.insert(Surface::xz_plane());
        let yz = stores.surfaces.insert(Surface::yz_plane());

        Self { xy, xz, yz }
    }

    /// Access the xy-plane
    pub fn xy(&self) -> Handle<Surface> {
        self.xy.clone()
    }

    /// Access the xz-plane
    pub fn xz(&self) -> Handle<Surface> {
        self.xz.clone()
    }

    /// Access the yz-plane
    pub fn yz(&self) -> Handle<Surface> {
        self.yz.clone()
    }
}
