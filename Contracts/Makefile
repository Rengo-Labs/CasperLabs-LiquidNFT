wasm_strip = wasm-strip target/wasm32-unknown-unknown/release/*.wasm 2>/dev/null | true

src_target = target/wasm32-unknown-unknown/release
liquid_factory_des_wasm = liquid-factory/liquid-factory-tests/wasm
liquid_locker_des_wasm = liquid-locker/liquid-locker-tests/wasm
liquid_helper_des_wasm = liquid-helper/liquid-helper-tests/wasm

prepare:
	rustup target add wasm32-unknown-unknown

build-contract-liquid-factory:
	cargo build --release -p liquid-factory -p cep47 -p erc20 --target wasm32-unknown-unknown && ${wasm_strip}
build-contract-liquid-helper:
	cargo build --release -p liquid-helper -p liquid-helper-proxy --target wasm32-unknown-unknown && ${wasm_strip}
build-contract-liquid-locker:
	cargo build --release -p liquid-locker -p liquid-locker-proxy -p cep47 -p erc20 --target wasm32-unknown-unknown && ${wasm_strip}

test-only-liquid-factory:
	cargo test -p liquid-factory-tests
test-only-liquid-helper:
	cargo test -p liquid-helper-tests
test-only-liquid-locker:
	cargo test -p liquid-locker-tests

copy-wasm-file-liquid-factory:
	cp ${src_target}/liquid-factory.wasm ${liquid_factory_des_wasm}
	cp ${src_target}/erc20-token.wasm ${liquid_factory_des_wasm}
	cp ${src_target}/cep47-token.wasm ${liquid_factory_des_wasm}
copy-wasm-file-liquid-helper:
	cp ${src_target}/liquid-helper.wasm ${liquid_helper_des_wasm}
	cp ${src_target}/liquid-helper-proxy.wasm ${liquid_helper_des_wasm}
copy-wasm-file-liquid-locker:
	cp ${src_target}/liquid-locker.wasm ${liquid_locker_des_wasm}
	cp ${src_target}/liquid-locker-proxy.wasm ${liquid_locker_des_wasm}
	cp ${src_target}/cep47-token.wasm ${liquid_locker_des_wasm}
	cp ${src_target}/erc20-token.wasm ${liquid_locker_des_wasm}


test-liquid-factory:
	make build-contract-liquid-factory && make copy-wasm-file-liquid-factory && make test-only-liquid-factory
test-liquid-helper:
	make build-contract-liquid-helper && make copy-wasm-file-liquid-helper && make test-only-liquid-helper
test-liquid-locker:
	make build-contract-liquid-locker && make copy-wasm-file-liquid-locker && make test-only-liquid-locker

all:
	make test-liquid-factory && make test-liquid-helper && make test-liquid-locker

clean:
	cargo clean
	rm -rf liquid-factory/liquid-factory-tests/wasm/*.wasm
	rm -rf liquid-helper/liquid-helper-tests/wasm/*.wasm
	rm -rf liquid-locker/liquid-locker-tests/wasm/*.wasm
	rm -rf Cargo.lock

lint: clippy
	cargo fmt --all

check-lint: clippy
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all -- -D warnings

git-clear:
	git rm -rf --cached .