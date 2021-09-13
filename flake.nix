{
  inputs = {
    fenix.url = github:nix-community/fenix;
    naersk.url = github:nmattia/naersk;
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    utils.url = github:numtide/flake-utils;
    move-tools.url = github:pontem-network/move-tools;
  };

  outputs = flake-args@{ self, nixpkgs, utils, naersk, fenix, move-tools, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustTargets = fenixArch.targets;
        llvmPackagesR = pkgs.llvmPackages_12;

        dove = move-tools.defaultPackage."${system}";

        toolchain = {
          channel = "nightly";
          date = "2021-04-24";
          sha256 = "sha256:0hsp3d521ri9h6xc2vjlqdiqkkzv95844wp2vv3a5hwcp157sykh";
        };

        rustToolchain = fenixArch.toolchainOf toolchain;
        rustToolchainWasm = rustTargets.wasm32-unknown-unknown.toolchainOf toolchain;

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
            dove

            (fenixArch.combine [
              (rustToolchain.withComponents [ "cargo" "clippy-preview" "llvm-tools-preview" "rust-std" "rustc" "rustc-dev" "rustfmt-preview" ])
              rustToolchainWasm.toolchain
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
