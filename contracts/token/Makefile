default: build

all: test

test: build
	cargo test

build:
	soroban contract build
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
