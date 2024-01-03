export RUSTDOCFLAGS := "-D warnings"

# Run all tests, including end-to-end export validation tests.
#
# This command is designed to be used regularly during CAD kernel development.
# It runs tests to check for code correctness, but accepts warnings in the code
# and does not check formatting or documentation.
#
# For a full build that mirrors the CI build, see `just ci`.
test:
    cargo test --all-features
    cargo run --package export-validator

# Run a full build that mirrors the CI build
#
# Basically, if this passes locally, then the CI build should pass too, once you
# submit your work as a pull request. Please note that the CI build is
# maintained separately from this file. Most deviations in behavior should
# probably be considered a bug in this file.
ci: test
    cargo fmt --check
    cargo clippy --workspace --all-features -- -D warnings
    cargo doc --no-deps --document-private-items --all-features --workspace
    cargo run --package cross-compiler
