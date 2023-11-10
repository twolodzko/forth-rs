test:
    cargo clippy
    cargo test

coverage:
    cargo tarpaulin --out Html

docs:
    cargo doc --no-deps --open

repl:
    RUST_BACKTRACE=1 cargo run --
