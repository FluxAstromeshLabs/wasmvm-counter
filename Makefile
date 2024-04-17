build:
	@cargo build --target wasm32-unknown-unknown

build-release:
	@cargo build --target wasm32-unknown-unknown --release
