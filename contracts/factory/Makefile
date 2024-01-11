default: build

all: test

RUST_VERSION := $(shell rustc --version | cut -d ' ' -f 2)

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	$(MAKE) -C ../pair || break;
	cargo +$(RUST_VERSION) build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_factory.wasm 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
