export RUSTDOCFLAGS := "-D warnings"

# Run a full build that mirrors the CI build
#
# Basically, if this passes locally, then the CI build should pass too, once you
# submit your work as a pull request. Please note that the CI build is
# maintained separately from this file. Most deviations in behavior should
# probably be considered a bug in this file.
ci:
    cargo fmt --check
    cargo clippy --all-features -- -D warnings
    cargo test --all-features
    cargo doc --no-deps --document-private-items --all-features --workspace
    cargo run --package cross-compiler
    cargo run --package export-validator
