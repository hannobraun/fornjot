//! # Fornjot - Experiment 2024-10-30
//!
//! Please check out the accompanying `README.md` file for high-level
//! documentation.
//!
//! As for the details, you are in the right place! I recommend you get started
//! with the [`geometry`] module, as that is the core of what this experiment is
//! about.

#![allow(clippy::module_inception)]

mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;

fn main() -> anyhow::Result<()> {
    let mut ops = geometry::OpsLog::default();
    model::model(&mut ops);

    export::export(&ops)?;
    app::run(ops)?;

    Ok(())
}
