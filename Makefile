wasm_strip = wasm-strip target/wasm32-unknown-unknown/release/*.wasm 2>/dev/null | true

src_target = target/wasm32-unknown-unknown/release
liquid_factory_address = Contracts/liquid-factory/liquid-factory-tests
liquid_locker_address = Contracts/liquid-locker/liquid-locker-tests
js_client_lnft_factory_address =  JsClients/LiquidNFT-Factory/LiquidNFT-Factory
js_client_lnft_factory-flow-tests_address= JsClients/LiquidNFT-Factory-Tests-Scripts
js_client_lnft_address =  JsClients/LiquidNFT/LiquidNFT
js_client_cep47_address = JsClients/casper-cep47
js_client_erc20_address = JsClients/uniswapV2Core-erc20/ERC20
js_client_uniswapV2Core-erc20 = JsClients/uniswapV2Core-erc20
js_client_main_contract_flow_script = JsClients/LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript
js_client_too_late_liquidate_flow_script = JsClients/LiquidNFT-Factory-Tests-Scripts/tooLateLiquidateFlowScript
js_client_for_frontend_liquid_nft = JsClientsForFrontend/LiquidNFT
js_client_for_frontend_liquid_nft_factory = JsClientsForFrontend/LiquidNFT-Factory
js_client_address = JsClients
LiquidNFT_node_modules = JsClients/LiquidNFT
LiquidNFT_factory_node_modules = JsClients/LiquidNFT-Factory

prepare:
	rustup target add wasm32-unknown-unknown

build-contract-liquid-factory:
	cargo build --release -p liquid-factory -p cep47 -p erc20 --target wasm32-unknown-unknown && ${wasm_strip}
build-contract-liquid-locker:
	cargo build --release -p liquid-locker -p cep47 -p erc20 --target wasm32-unknown-unknown && ${wasm_strip}

test-only-liquid-factory:
	cargo test -p liquid-factory-tests

copy-wasm-file-liquid-factory:
	cp ${src_target}/liquid-factory.wasm ${liquid_factory_address}/wasm
	cp ${src_target}/erc20-token.wasm ${liquid_factory_address}/wasm
	cp ${src_target}/cep47-token.wasm ${liquid_factory_address}/wasm

copy-wasm-file-js-client:
	cp ${src_target}/liquid-factory.wasm ${js_client_lnft_factory_address}/wasm
	cp ${src_target}/erc20-token.wasm ${js_client_erc20_address}/wasm
	cp ${src_target}/cep47-token.wasm ${js_client_cep47_address}/wasm
	

test-liquid-factory:
	make build-contract-liquid-factory && make copy-wasm-file-liquid-factory && make test-only-liquid-factory

all:
	make build-contract-liquid-factory && make build-contract-liquid-locker

all-jsclient:
	make build-contract-liquid-factory && make build-contract-liquid-locker && make copy-wasm-file-js-client && make generate-key && make build-jsclients

test-all:
	make all
	make test-liquid-factory

clean:
	cargo clean
	rm -rf ${liquid_factory_address}/wasm/*.wasm
	rm -rf ${liquid_locker_address}/wasm/*.wasm
	rm -rf ${js_client_lnft_address}/wasm/*.wasm
	rm -rf ${js_client_cep47_address}/wasm/*.wasm
	rm -rf ${js_client_cep47_address}/keys
	rm -rf ${js_client_lnft_address}/keys
	rm -rf ${js_client_lnft_factory_address}/keys
	rm -rf ${js_client_erc20_address}/keys
	rm -rf ${js_client_erc20_address}/wasm/*.wasm
	rm -rf ${js_client_lnft_factory-flow-tests_address}/keys
	rm -rf ${js_client_main_contract_flow_script}/keys
	rm -rf ${js_client_too_late_liquidate_flow_script}/keys
	rm -rf ${js_client_address}	
	rm -rf ${LiquidNFT_factory_node_modules}/node_modules
	rm -rf ${js_client_cep47_address}/node_modules
	rm -rf ${LiquidNFT_node_modules}/node_modules
	rm -rf ${js_client_uniswapV2Core-erc20}/node_modules
	rm -rf ${js_client_main_contract_flow_script}/node_modules
	rm -rf ${js_client_too_late_liquidate_flow_script}/node_modules
	rm -rf ${js_client_for_frontend_liquid_nft}/node_modules
	rm -rf ${js_client_for_frontend_liquid_nft_factory}/node_modules
	rm -rf keys
	rm -rf Cargo.lock

generate-key:
	rm -rf keys
	rm -rf ${js_client_address}/keys	
	rm -rf ${js_client_cep47_address}/keys
	rm -rf ${js_client_lnft_address}/keys
	rm -rf ${js_client_lnft_factory_address}/keys
	rm -rf ${js_client_erc20_address}/keys
	rm -rf ${js_client_lnft_factory-flow-tests_address}/keys
	rm -rf ${js_client_main_contract_flow_script}/keys
	rm -rf ${js_client_too_late_liquidate_flow_script}/keys
	casper-client keygen ./keys
	cp -R ./keys ${js_client_cep47_address} 
	cp -R ./keys ${js_client_lnft_factory_address}
	cp -R ./keys ${js_client_lnft_factory-flow-tests_address}
	cp -R ./keys ${js_client_lnft_address}
	cp -R ./keys ${js_client_erc20_address}
	cp -R ./keys ${js_client_main_contract_flow_script}
	cp -R ./keys ${js_client_too_late_liquidate_flow_script}
	cp -R ./keys ${js_client_address}

lint: clippy
	cargo fmt --all

check-lint: clippy
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all -- -D warnings

git-clear:
	git rm -rf --cached .
	git add .


build-jsclients:
	cd ${js_client_lnft_factory_address}/ && npm cache clean --force && npm ci
	cd ${js_client_cep47_address}/ && npm cache clean --force && npm ci
	cd ${js_client_lnft_address}/ && npm cache clean --force && npm ci
	cd ${js_client_uniswapV2Core-erc20}/ && npm cache clean --force && npm ci
	cd ${js_client_main_contract_flow_script}/&& npm cache clean --force && npm ci
	cd ${js_client_too_late_liquidate_flow_script}/&& npm cache clean --force && npm ci
	cd ${js_client_for_frontend_liquid_nft}/&& npm cache clean --force && npm ci
	cd ${js_client_for_frontend_liquid_nft_factory}/&& npm cache clean --force && npm ci