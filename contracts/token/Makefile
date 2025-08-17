default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm
	stellar contract optimize --wasm target/wasm32v1-none/release/soroban_token_contract.wasm


fmt:
	cargo fmt --all

clean:
	cargo clean
