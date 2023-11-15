test: lint unit-test integration-test

lint:
    cargo clippy

unit-test:
    cargo test

integration-test:
    cargo run -- \
        include/std.f \
        examples/testsuite.f >/dev/null
    cargo run -- \
        include/std.f \
        examples/chars.f \
        examples/fact.f \
        examples/fizzbuzz.f \
        examples/hello.f \
        examples/integers.f \
        examples/leap.f \
        examples/pascal.f \
        examples/stars.f \
        >/dev/null

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
