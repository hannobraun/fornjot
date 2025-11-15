# Developer Docs

The contents of this directory are the rendered Rustdoc pages for every crate, tool, and example
model in the Fornjot workspace. They are generated with `cargo doc --workspace` and then copied
here (along with the shared assets) so they can be browsed locally or hosted as static files without
depending on the build output in `target/doc`.

## Getting Started

Open `developer_docs/index.html` in your browser. That landing page uses the same rustdoc styling as
the crate pages and links into each crate/tool/model, so all navigation stays relative and the
pre-bundled CSS/JS/fonts keep working. Once inside a crate page, use the sidebar search to jump
directly to items, or follow the breadcrumbs to explore the module hierarchy.

## What You’ll Find

- **Primary Crates** – `fj`, `fj_core`, `fj_math`, `fj_export`, `fj_interop`, `fj_viewer`, and
  `fj_window`, covering the core kernel and user-facing APIs.
- **Tools** – automation utilities like `automator`, `release_operator`, and build helpers.
- **Models & Examples** – reference models such as `all`, `star`, `split`, and `vertices_indices`
  that show how to drive the engine.

Each crate folder mirrors the standard rustdoc structure (e.g. `index.html`, `all.html`,
`sidebar-items.js`). Because the assets in `static.files/` live alongside these pages, everything
renders with the normal rustdoc theme even when viewed offline.
