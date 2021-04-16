.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --all
	SKIP_WASM_BUILD=1 cargo check --all --tests
	pushd node && cargo check --features=runtime-benchmarks; popd

.PHONY: clippy
clippy:
	# Build with target=wasm32 as workaround for substrate issue
	pushd pallets/sp-mvm && \
	cargo clippy -p=sp-mvm --target=wasm32-unknown-unknown --no-default-features

.PHONY: bench
bench:
	# This is just an example about how to run benchmarks for the pallet
	mkdir -p ./target/sp-bench
	pushd node && \
	cargo run --release --features=runtime-benchmarks -- \
		benchmark \
		--dev \
		-lsp_mvm=trace \
		--pallet=sp_mvm \
		--extrinsic='*' \
		--execution=wasm \
		--wasm-execution=compiled \
		--steps=20 --repeat=10 \
		--output=../target/sp-bench

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all --no-fail-fast -- --nocapture --test-threads=1

.PHONY: run
run:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo run --release -- --dev --tmp -lsp_mvm=trace

.PHONY: build
build:
	WASM_BUILD_TOOLCHAIN=`cat rust-toolchain` cargo build --release

.PHONY: assets
assets:
	pushd pallets/sp-mvm/tests/assets && ./build_assets.sh
	pushd pallets/sp-mvm/tests/benchmark_assets && ./build_assets.sh
