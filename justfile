build:
    cargo clippy --all-features
    cargo test --all-features
    cargo fmt --check
    cargo run -p export-validator
