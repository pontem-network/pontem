.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all --no-fail-fast -- --nocapture

.PHONY: run
run:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo run --release -- --dev --tmp

.PHONY: build
build:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo build --release
