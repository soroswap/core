default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	cargo build --target wasm32v1-none --release
	stellar contract optimize --wasm target/wasm32v1-none/release/soroswap_pair.wasm  
	@ls -l target/wasm32v1-none/release/*.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
