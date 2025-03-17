//! # Fornjot - Experiment 2024-12-09
//!
//! Please check the accompanying `README.md` for context and high-level
//! documentation.
//!
//! As for the details, you're in the right place! I'd start with the [`model`],
//! [`topology`], [`operations`], and [`geometry`] modules, roughly in that
//! order. Those are core to the CAD stuff, while the rest is mostly there for
//! support.

#![allow(clippy::module_inception)]

mod app;
mod export;
mod extra;
mod geometry;
mod math;
mod model;
mod object;
mod operations;
mod render;
mod topology;
mod view;

fn main() -> anyhow::Result<()> {
    let model = model::model();

    export::export(&model)?;
    app::run(model)?;

    Ok(())
}
