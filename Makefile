SHELL := /usr/bin/env bash

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check: check-all check-no-std check-benchmarks

.PHONY: check-all
check-all: assets
	export SKIP_WASM_BUILD=1
	cargo check --all
	cargo check --all --tests

.PHONY: check-benchmarks
check-benchmarks: assets
	export SKIP_WASM_BUILD=1
	cargo check --features=runtime-benchmarks

.PHONY: check-no-std
check-no-std: assets
	cargo check -p=sp-mvm --target=wasm32-unknown-unknown --no-default-features

.PHONY: bench-rename-modules
bench-rename-modules: assets
	scripts/rename_modules.sh

.PHONY: clippy
clippy:
	cargo clippy -p=sp-mvm -p=sp-mvm-rpc -p=sp-mvm-rpc-runtime
	cargo clippy -p=sp-mvm --target=wasm32-unknown-unknown --no-default-features

.PHONY: bench
bench: target/bench-bin
	./target/bench-bin \
		benchmark \
		--dev \
		-lsp_mvm=trace \
		--pallet=sp_mvm \
		--extrinsic='*' \
		--execution=wasm \
		--wasm-execution=compiled \
		--steps=20 --repeat=10 \
		--output=target

.PHONY: Run groupsign benchmarks
run-bench-groupsign: target/bench-bin
	./target/bench-bin \
		benchmark \
		--dev \
		--pallet=groupsign \
		--extrinsic='*' \
		--execution=wasm \
		--wasm-execution=compiled \
		--steps=20 --repeat=10 \
		--output=target/gs-bench

.PHONY: Build a benchmarking binary
target/bench-bin: assets
	# Checking whether benchlib is up to date
	diff -e pallets/groupsign/src/benchmarking/benchlib.rs pallets/groupsign/canned-benchmarks/src/benchlib.rs || { \
		echo " !!!!!!! BENCHLIB.RS IS NOT SYNCHRONIZED"; \
		echo " !!!!!!! run this and push to repo:"; \
		echo cp pallets/groupsign/canned-benchmarks/src/benchlib.rs pallets/groupsign/src/benchmarking/benchlib.rs; \
		exit 1; \
	}
	
	cargo build \
		--release \
		--bin pontem \
		--features=runtime-benchmarks
	install target/release/libsp_mvm.rlib ./target/bench-bin

.PHONY: test
test: assets
	export SKIP_WASM_BUILD=1
	cargo test --all --no-fail-fast -- --nocapture --test-threads=1

.PHONY: run
run:
	export WASM_BUILD_TOOLCHAIN=$(cat rust-toolchain)
	cargo run --release -- --dev --tmp -lsp_mvm=trace

.PHONY: build
build:
	export WASM_BUILD_TOOLCHAIN=`cat rust-toolchain`
	cargo build --release

.PHONY: build-bench
build-bench: assets
	export SKIP_WASM_BUILD=0
	export WASM_BUILD_TOOLCHAIN=`cat rust-toolchain`
	cargo build --release --features=runtime-benchmarks

.PHONY: assets
assets: pallets/sp-mvm/tests/assets/stdlib pallets/sp-mvm/tests/benchmark_assets/stdlib runtime/src/tests/assets/stdlib

.PHONY: clean-assets
clean-assets:
	rm -rf \
		pallets/sp-mvm/tests/assets/stdlib \
		pallets/sp-mvm/tests/benchmark_assets/stdlib \
		runtime/src/tests/assets/stdlib

	git clean -dfX -- \
		pallets/sp-mvm/tests

pallets/sp-mvm/tests/assets/stdlib:
	cd pallets/sp-mvm/tests/assets; ./build_assets.sh

pallets/sp-mvm/tests/benchmark_assets/stdlib:
	cd pallets/sp-mvm/tests/benchmark_assets; ./build_assets.sh

runtime/src/tests/assets/stdlib:
	cd runtime/src/tests/assets; ./build_assets.sh

.PHONY: coverage
coverage: assets
	export SKIP_WASM_BUILD=1
	export CARGO_INCREMENTAL=0
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
	# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"
	export RUSTDOCFLAGS="-Cpanic=abort"
	# cargo test -p=sp-mvm --no-fail-fast -- --test-threads=1
	cargo test --no-fail-fast -- --test-threads=1
	grcov . \
		-s . \
		--binary-path ./target/debug/ \
		--guess-directory-when-missing \
		--llvm \
		--branch \
		--ignore-not-existing \
		--filter covered \
		-o ./target/debug/coverage/
	# to produce html report add:
	# -t html
