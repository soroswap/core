default: build

all: test

test: build
	cargo test

build:
	soroban contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm


fmt:
	cargo fmt --all

clean:
	cargo clean
