{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    rocksdb = {
      flake = false;
      url = "github:facebook/rocksdb";
    };
  };

  outputs = flake-args@{ self, nixpkgs, utils, naersk, fenix, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustTargets = fenixArch.targets;
        llvmPackagesR = pkgs.llvmPackages_12;

        rustToolchain = fenixArch.stable;
        rustToolchainWasm = rustTargets.wasm32-unknown-unknown.latest;

        naersk-lib = naersk.lib.${system}.override {
          cargo = rustToolchain.toolchain;
          rustc = rustToolchain.toolchain;
        };

      in {

        devShell =
        with pkgs; mkShell {
          buildInputs = [
            protobuf openssl pre-commit pkgconfig
            llvmPackagesR.clang

            (fenixArch.combine [
              (fenixArch.latest.withComponents [ "cargo" "clippy-preview" "llvm-tools-preview" "rust-std" "rustc" "rustc-dev" "rustfmt-preview" ])
              rustTargets.wasm32-unknown-unknown.latest.toolchain
            ])

          ];

#          SKIP_WASM_BUILD = 1;
          PROTOC = "${protobuf}/bin/protoc";
          # PROTOC_INCLUDE="${protobuf}/include";
          LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/library/";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };

      });

}
