SHELL := /bin/bash

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --all

.PHONY: clippy
clippy:
	SKIP_WASM_BUILD=1 cargo clippy -p=sp-mvm

.PHONY: test
test:
	pushd pallets/sp-mvm/tests/assets; ./build_assets.sh
	SKIP_WASM_BUILD=1 cargo test --all --no-fail-fast -- --nocapture --test-threads=1

.PHONY: run
run:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo run --release -- --dev --tmp -lsp_mvm=trace

.PHONY: build
build:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo build --release
