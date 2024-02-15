default: build

all: test

test: build
	cargo test

build: phoenix_build
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_aggregator.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

# New target for building phoenix
phoenix_build:
	@cd protocols/phoenix && make build

fmt:
	cargo fmt --all --check

clean:
	cargo clean