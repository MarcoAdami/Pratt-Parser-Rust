check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test

fix:
    cargo fmt --all
    cargo clippy --fix --allow-dirty