export RUSTDOCFLAGS := "-D warnings"

build:
    cargo fmt --check
    cargo clippy --all-features -- -D warnings
    cargo test --all-features
    cargo doc --no-deps --document-private-items --all-features --workspace
    cargo run --package cross-compiler
    cargo run --package export-validator
