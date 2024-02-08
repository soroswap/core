default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_aggregator.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
