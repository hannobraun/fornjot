//! # Rendering infrastructure
//!
//! This has mostly been inherited from the previous experiment, with some
//! extensions. The remainder of this documtation is mostly copied from that
//! previous experiment.
//!
//! Even though most of the work for this prototype went into the renderer, it
//! is not the most interesting aspect, and I'm not going to document it in
//! detail. It's a pretty basic architecture, optimized for the speed of having
//! written it, not speed of rendering.
//!
//! The most interesting aspect in terms of what this experiment could mean for
//! Fornjot, is that this renderer has a direct dependency on
//! [`geometry`](crate::geometry). Versus the current Fornjot renderer, which
//! only communicates with the CAD core through another interop crate.
//!
//! I'm pretty sure that whatever happens with these experiments, I'll go with
//! the simpler approach going forward. I'm not even sure anymore what the
//! thinking behind the original design was (it's been years).
//!
//! I probably overestimated the importance of making things pluggable, and
//! making parts of Fornjot usable in isolation. Going forward, I'm viewing the
//! renderer as something that is very purpose-built for the needs of developing
//! Fornjot. Not something I'd expect anybody building on top of Fornjot would
//! want to use, except maybe to get started.

mod geometry;
mod pipeline;
mod renderer;
mod text;
mod uniforms;
mod vertex;

pub use self::renderer::Renderer;
