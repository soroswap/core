default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	cargo build --target wasm32-unknown-unknown --release 
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_library.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
