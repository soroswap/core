default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	$(MAKE) -C ../pair || break;
	$(MAKE) -C ../factory || break;
	cargo +1.74.1 build --target wasm32-unknown-unknown --release 
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_router.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
