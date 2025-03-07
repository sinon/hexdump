set dotenv-load
build:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
b:build
format:
        @cargo fmt --version
        cargo fmt
lint:
        @cargo clippy --version
        cargo clippy -- -D warnings -W clippy::pedantic -W clippy::nursery
        cargo doc
test:
    cargo nextest run --all-targets --no-fail-fast

t:test
