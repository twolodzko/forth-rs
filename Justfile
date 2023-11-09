test:
    cargo clippy
    cargo test

docs:
    cargo doc --no-deps --open

repl:
    RUST_BACKTRACE=1 cargo run --
