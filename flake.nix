{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustTargets = fenixArch.targets;
        llvmPackages = pkgs.llvmPackages_12;

        rustToolchain = fenixArch.stable;
        rustToolchainWasm = rustTargets.wasm32-unknown-unknown.latest;

        naersk-lib = naersk.lib.${system}.override {
          cargo = rustToolchain.toolchain;
          rustc = rustToolchain.toolchain;
        };

      in {

        # defaultPackage = naersk-lib.buildPackage (with pkgs; {
        #   root = ./.;
        #   src = ./.;

        #   PROTOC = "${protobuf}/bin/protoc";
        #   PROTOC_INCLUDE="${protobuf}/include";
        #   LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
        #   LIBCLANG_PATH="${llvmPackages_11.clang-unwrapped.lib}/lib";
        #   RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/library/";

        #   SKIP_WASM_BUILD = 1;
        # });

        # defaultApp = utils.mkApp {
        #     drv = self.defaultPackage."${system}";
        # };

        devShell =
        with pkgs; mkShell {
          buildInputs = [
            protobuf openssl pre-commit pkgconfig
            llvmPackages.libclang.out
            cargo-binutils

            (fenixArch.combine [
              fenixArch.stable.completeToolchain
              rustTargets.wasm32-unknown-unknown.stable.toolchain
              # rustToolchainWasm.toolchain
            ])

          ];

#          SKIP_WASM_BUILD = 1;
          PROTOC = "${protobuf}/bin/protoc";
          PROTOC_INCLUDE="${protobuf}/include";
          LLVM_CONFIG_PATH="${llvmPackages.bintools}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackages_11.clang-unwrapped.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain.rust-src}/lib/rustlib/src/rust/library/";

        };

      });

}
