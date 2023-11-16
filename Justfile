test: lint unit-test examples

lint:
    cargo clippy

unit-test:
    cargo test

examples:
    cargo run -- include/std.f examples/* >/dev/null

coverage:
    cargo tarpaulin --out Html

benchmark:
    hyperfine -N -r 1000 \
        'gforth examples/testsuite.f' \
        'forthrs include/std.f examples/testsuite.f'

docs:
    cargo doc --no-deps --open

repl:
    RUST_BACKTRACE=1 cargo run --

install:
    cargo install --path .

lines:
    @ find . -type f -name "*.rs" -not -name "test*" -exec awk '1;/#[cfg\(test\)]/{exit}' {} \; | grep . | wc -l

clean:
    rm -rf target/
