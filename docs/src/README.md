# Developer Documentation

The Fornjot developer documentation is generated with `cargo doc --workspace`. Only the
source files that describe how the docs are built are checked into Git. The rendered HTML,
JavaScript, CSS, and font assets stay outside the repository and are rebuilt automatically
in CI so that pull requests stay fast and reviewable.

## Building Locally

1. Make sure you have Rust installed (see `rust-toolchain.toml`).
2. Run `cargo doc --workspace --all-features --no-deps` from the repository root.
3. Open `target/doc/index.html` (or a specific crate such as `target/doc/fj/index.html`) in
   your browser.

The command uses the shared workspace so that every crate, tool, and example model is rendered
with consistent metadata and search indices.

## Deployment

The GitHub Actions workflow in `.github/workflows/docs.yml` is responsible for building the
same docs on every push to `main` and publishing the result to the `gh-pages` branch. GitHub
Pages is configured to serve that branch so the developer docs stay up-to-date without
committing generated files.

## About the Legacy `developer_docs/` folder

Older versions of the repository committed the pre-rendered docs to `developer_docs/`. That
folder is now ignored. If you still have it locally, you can keep using it for offline
reference, but new builds should be produced via `cargo doc` so that CI and GitHub Pages
match what you see locally.
