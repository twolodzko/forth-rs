test:
    cargo clippy
    cargo test
    # cargo run -- examples/* >/dev/null
    cargo run -- examples/hello.f examples/fizzbuzz.f >/dev/null

coverage:
    cargo tarpaulin --out Html

docs:
    cargo doc --no-deps --open

repl:
    RUST_BACKTRACE=1 cargo run --

install:
    cargo install --path .

lines:
    @ find . -type f -name "*.rs" -not -name "test*" -exec awk '1;/#[cfg\(test\)]/{exit}' {} \; | grep . | wc -l
