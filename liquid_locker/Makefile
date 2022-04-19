prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p liquid_locker --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquid_locker.wasm 2>/dev/null | true

test-only:
	cargo test -p liquid_locker-tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm liquid_locker-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf liquid_locker-tests/wasm/*.wasm
