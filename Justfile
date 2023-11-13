test:
    cargo clippy
    cargo test

coverage:
    cargo tarpaulin --out Html

docs:
    cargo doc --no-deps --open

repl:
    RUST_BACKTRACE=1 cargo run --

lines:
    @ find . -type f -name "*.rs" -not -name "test*" -exec awk '1;/#[cfg\(test\)]/{exit}' {} \; | grep . | wc -l
