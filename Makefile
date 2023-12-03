.PHONY: lint

lint:
	cargo clippy --target=wasm32-wasi

fix-formatting:
	cargo fmt

check-formatting:
	cargo fmt --check
