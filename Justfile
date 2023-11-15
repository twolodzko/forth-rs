test: lint unit-test integration-test

lint:
    cargo clippy

unit-test:
    cargo test

integration-test:
    cargo run -- \
        include/std.f \
        examples/* \
        >/dev/null

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
