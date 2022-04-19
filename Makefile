src_target = target/wasm32-unknown-unknown/release

prepare:
	rustup target add wasm32-unknown-unknown

build-contract-liquid-helper:
	cargo build --release -p liquid-helper --target wasm32-unknown-unknown
build-contract-liquid-locker:
	cargo build --release -p liquid-locker -p liquid-locker-proxy --target wasm32-unknown-unknown
	cargo build --release -p liquid-locker-proxy --target wasm32-unknown-unknown

test-only-liquid-helper:
	cargo test -p liquid-helper-tests
test-only-liquid-locker:
	cargo test -p liquid-locker-tests

copy-wasm-file-liquid-helper:
	cp ${src_target}/liquid-helper.wasm liquid-helper/liquid-helper-tests/wasm
copy-wasm-file-liquid-locker:
	cp ${src_target}/liquid-locker.wasm liquid-locker/liquid-locker-tests/wasm
	cp ${src_target}/liquid-locker-proxy.wasm liquid-locker/liquid-locker-tests/wasm

test-liquid-helper:
	make build-contract-liquid-helper && make copy-wasm-file-liquid-helper && make test-only-liquid-helper
test-liquid-locker:
	make build-contract-liquid-locker && make copy-wasm-file-liquid-locker && make test-only-liquid-locker

test-all:
	make test-liquid-helper && make test-liquid-locker

clean:
	cargo clean
	rm -rf liquid-helper/liquid-helper-tests/wasm/*.wasm
	rm -rf liquid-locker/liquid-locker-tests/wasm/*.wasm