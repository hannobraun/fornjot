export RUSTDOCFLAGS := "-D warnings"

build:
    cargo clippy --all-features
    cargo test --all-features
    cargo fmt --check
    cargo doc --no-deps --document-private-items --all-features --workspace
    cargo run -p export-validator
