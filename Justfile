test:
    cargo clippy
    cargo test

docs:
    cargo doc --no-deps --open
