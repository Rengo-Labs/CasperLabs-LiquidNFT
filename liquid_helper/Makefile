prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p liquid_helper --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquid_helper.wasm 2>/dev/null | true

test-only:
	cargo test -p liquid_helper-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm liquid_helper-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf liquid_helper-tests/wasm/*.wasm
